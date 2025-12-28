macro_rules! deps {
    () => {
        Engine!();
    };
}

macro_rules! encode_with_padding {
    () => {
        deps!();
        # [doc = " B64-encode and pad (if configured)."] # [doc = ""] # [doc = " This helper exists to avoid recalculating `encoded_size`, which is relatively expensive on short"] # [doc = " inputs."] # [doc = ""] # [doc = " `encoded_size` is the encoded size calculated for `input`."] # [doc = ""] # [doc = " `output` must be of size `encoded_size`."] # [doc = ""] # [doc = " All bytes in `output` will be written to since it is exactly the size of the output."] pub (crate) fn encode_with_padding < E : Engine + ? Sized > (input : & [u8] , output : & mut [u8] , engine : & E , expected_encoded_size : usize ,) { debug_assert_eq ! (expected_encoded_size , output . len ()) ; let b64_bytes_written = engine . internal_encode (input , output) ; let padding_bytes = if engine . config () . encode_padding () { add_padding (b64_bytes_written , & mut output [b64_bytes_written ..]) } else { 0 } ; let encoded_bytes = b64_bytes_written . checked_add (padding_bytes) . expect ("usize overflow when calculating b64 length") ; debug_assert_eq ! (expected_encoded_size , encoded_bytes) ; }
    };
}

encode_with_padding!()