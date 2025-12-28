macro_rules! deps {
    () => {
        Renderer!();
    };
}

macro_rules! maybe_anonymized {
    () => {
        deps!();
        fn maybe_anonymized (renderer : & Renderer , line_num : usize , max_line_num_len : usize) -> String { format ! ("{:>max_line_num_len$}" , if renderer . anonymized_line_numbers { Cow :: Borrowed (ANONYMIZED_LINE_NUM) } else { Cow :: Owned (line_num . to_string ()) }) }
    };
}

maybe_anonymized!()