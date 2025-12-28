macro_rules! deps {
    () => {
        Result!();
        Error!();
        CsvTab!();
    };
}

macro_rules! impl_622 {
    () => {
        deps!();
        impl CsvTab { fn reader (& self) -> Result < csv :: Reader < File > , csv :: Error > { csv :: ReaderBuilder :: new () . has_headers (self . has_headers) . delimiter (self . delimiter) . quote (self . quote) . from_path (& self . filename) } fn parse_byte (arg : & str) -> Option < u8 > { if arg . len () == 1 { arg . bytes () . next () } else { None } } }
    };
}

impl_622!()