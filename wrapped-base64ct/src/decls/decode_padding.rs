macro_rules! deps {
    () => {
        InvalidEncodingError!();
    };
}

macro_rules! decode_padding {
    () => {
        deps!();
        # [doc = " Validate padding is of the expected length compute unpadded length."] # [doc = ""] # [doc = " Note that this method does not explicitly check that the padded data"] # [doc = " is valid in and of itself: that is performed by `validate_last_block` as a"] # [doc = " final step."] # [doc = ""] # [doc = " Returns length-related errors eagerly as a [`Result`], and data-dependent"] # [doc = " errors (i.e. malformed padding bytes) as `i16` to be combined with other"] # [doc = " encoding-related errors prior to branching."] # [inline (always)] pub (crate) fn decode_padding (input : & [u8]) -> Result < (usize , i16) , InvalidEncodingError > { if input . len () % 4 != 0 { return Err (InvalidEncodingError) ; } let unpadded_len = match * input { [.. , b0 , b1] => is_pad_ct (b0) . checked_add (is_pad_ct (b1)) . and_then (| len | len . try_into () . ok ()) . and_then (| len | input . len () . checked_sub (len)) . ok_or (InvalidEncodingError) ? , _ => input . len () , } ; let padding_len = input . len () . checked_sub (unpadded_len) . ok_or (InvalidEncodingError) ? ; let err = match * input { [.. , b0] if padding_len == 1 => is_pad_ct (b0) ^ 1 , [.. , b0 , b1] if padding_len == 2 => (is_pad_ct (b0) & is_pad_ct (b1)) ^ 1 , _ => { if padding_len == 0 { 0 } else { return Err (InvalidEncodingError) ; } } } ; Ok ((unpadded_len , err)) }
    };
}

decode_padding!()