macro_rules! percent_encode {
    () => {
        fn percent_encode (s : & str) -> Cow < '_ , str > { percent_encoding :: utf8_percent_encode (s , percent_encoding :: NON_ALPHANUMERIC) . into () }
    };
}

percent_encode!()