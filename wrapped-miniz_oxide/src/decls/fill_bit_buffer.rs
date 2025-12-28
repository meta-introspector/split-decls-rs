macro_rules! deps {
    () => {
        InputWrapper!();
        BitBuffer!();
        LocalVars!();
    };
}

macro_rules! fill_bit_buffer {
    () => {
        deps!();
        # [doc = " Same as previous, but for non-64-bit platforms."] # [doc = " Ensures at least 16 bits are present, requires at least 2 bytes in the in buffer."] # [inline (always)] # [cfg (not (target_pointer_width = "64"))] fn fill_bit_buffer (l : & mut LocalVars , in_iter : & mut InputWrapper) { if l . num_bits < 15 { l . bit_buf |= BitBuffer :: from (read_u16_le (in_iter)) << l . num_bits ; l . num_bits += 16 ; } }
    };
}

fill_bit_buffer!();