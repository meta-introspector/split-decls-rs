macro_rules! encoded_len {
    () => {
        # [doc = " Calculate the base64 encoded length for a given input length, optionally including any"] # [doc = " appropriate padding bytes."] # [doc = ""] # [doc = " Returns `None` if the encoded length can't be represented in `usize`. This will happen for"] # [doc = " input lengths in approximately the top quarter of the range of `usize`."] # [must_use] pub const fn encoded_len (bytes_len : usize , padding : bool) -> Option < usize > { let rem = bytes_len % 3 ; let complete_input_chunks = bytes_len / 3 ; let complete_chunk_output = if let Some (complete_chunk_output) = complete_input_chunks . checked_mul (4) { complete_chunk_output } else { return None ; } ; if rem > 0 { if padding { complete_chunk_output . checked_add (4) } else { let encoded_rem = match rem { 1 => 2 , _ => 3 , } ; complete_chunk_output . checked_add (encoded_rem) } } else { Some (complete_chunk_output) } }
    };
}

encoded_len!();