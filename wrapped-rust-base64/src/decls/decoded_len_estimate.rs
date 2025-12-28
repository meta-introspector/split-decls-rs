macro_rules! decoded_len_estimate {
    () => {
        # [doc = " Returns a conservative estimate of the decoded size of `encoded_len` base64 symbols (rounded up"] # [doc = " to the next group of 3 decoded bytes)."] # [doc = ""] # [doc = " The resulting length will be a safe choice for the size of a decode buffer, but may have up to"] # [doc = " 2 trailing bytes that won't end up being needed."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use base64::decoded_len_estimate;"] # [doc = ""] # [doc = " assert_eq!(3, decoded_len_estimate(1));"] # [doc = " assert_eq!(3, decoded_len_estimate(2));"] # [doc = " assert_eq!(3, decoded_len_estimate(3));"] # [doc = " assert_eq!(3, decoded_len_estimate(4));"] # [doc = " // start of the next quad of encoded symbols"] # [doc = " assert_eq!(6, decoded_len_estimate(5));"] # [doc = " ```"] # [must_use] pub fn decoded_len_estimate (encoded_len : usize) -> usize { STANDARD . internal_decoded_len_estimate (encoded_len) . decoded_len_estimate () }
    };
}

decoded_len_estimate!()