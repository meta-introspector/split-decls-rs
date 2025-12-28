macro_rules! ParserNumber {
    () => {
        pub (crate) enum ParserNumber { F64 (f64) , U64 (u64) , I64 (i64) , # [cfg (feature = "arbitrary_precision")] String (String) , }
    };
}

ParserNumber!();