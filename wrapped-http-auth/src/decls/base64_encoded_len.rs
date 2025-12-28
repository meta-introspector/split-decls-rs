macro_rules! base64_encoded_len {
    () => {
        # [doc = " Returns the base64-encoded length for the given input length, including padding."] fn base64_encoded_len (input_len : usize) -> usize { (input_len + 2) / 3 * 4 }
    };
}

base64_encoded_len!()