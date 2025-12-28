macro_rules! write_label_len {
    () => {
        # [doc = " Returns the total number of bytes (including padding) that would be written"] # [doc = " for the given label. This panics if the given label contains a NUL byte or"] # [doc = " is longer than 255 bytes. (The size restriction exists so that searching"] # [doc = " for a label during deserialization can be done in small bounded space.)"] pub (crate) fn write_label_len (label : & str) -> usize { assert ! (label . len () <= 255 , "label must not be longer than 255 bytes") ; assert ! (label . bytes () . all (| b | b != 0) , "label must not contain NUL bytes") ; let label_len = label . len () + 1 ; label_len + padding_len (label_len) }
    };
}

write_label_len!()