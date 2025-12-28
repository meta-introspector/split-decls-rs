macro_rules! deps {
    () => {
        Action!();
        DecompressorOxide!();
        BitBuffer!();
        LocalVars!();
        InputWrapper!();
    };
}

macro_rules! decode_huffman_code {
    () => {
        deps!();
        # [doc = " Try to decode the next huffman code, and puts it in the counter field of the decompressor"] # [doc = " if successful."] # [doc = ""] # [doc = " # Returns"] # [doc = " The specified action returned from `f` on success,"] # [doc = " `Action::End` if there are not enough data left to decode a symbol."] fn decode_huffman_code < F > (r : & mut DecompressorOxide , l : & mut LocalVars , table : usize , flags : u32 , in_iter : & mut InputWrapper , f : F ,) -> Action where F : FnOnce (& mut DecompressorOxide , & mut LocalVars , i32) -> Action , { if l . num_bits < 15 { if in_iter . bytes_left () < 2 { loop { let mut temp = i32 :: from (r . tables [table] . fast_lookup (l . bit_buf)) ; if temp >= 0 { let code_len = (temp >> 9) as u32 ; if (code_len != 0) && (l . num_bits >= code_len) { break ; } } else if l . num_bits > FAST_LOOKUP_BITS . into () { let mut code_len = u32 :: from (FAST_LOOKUP_BITS) ; loop { temp = i32 :: from (r . tables [table] . tree [(! temp + ((l . bit_buf >> code_len) & 1) as i32) as usize] ,) ; code_len += 1 ; if temp >= 0 || l . num_bits < code_len + 1 { break ; } } if temp >= 0 { break ; } } let mut byte = 0 ; if let a @ Action :: End (_) = read_byte (in_iter , flags , | b | { byte = b ; Action :: None }) { return a ; } ; l . bit_buf |= BitBuffer :: from (byte) << l . num_bits ; l . num_bits += 8 ; if l . num_bits >= 15 { break ; } } } else { l . bit_buf |= BitBuffer :: from (read_u16_le (in_iter)) << l . num_bits ; l . num_bits += 16 ; } } let mut symbol = i32 :: from (r . tables [table] . fast_lookup (l . bit_buf)) ; let code_len ; if symbol >= 0 { code_len = (symbol >> 9) as u32 ; symbol &= 511 ; } else { let res = r . tables [table] . tree_lookup (symbol , l . bit_buf , FAST_LOOKUP_BITS) ; symbol = res . 0 ; code_len = res . 1 ; } ; l . bit_buf >>= code_len ; l . num_bits -= code_len ; f (r , l , symbol) }
    };
}

decode_huffman_code!();