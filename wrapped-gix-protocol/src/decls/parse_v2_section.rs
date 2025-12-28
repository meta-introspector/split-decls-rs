macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! parse_v2_section {
    () => {
        deps!();
        fn parse_v2_section < 'a , T > (line : & mut String , reader : & mut impl ExtendedBufRead < 'a > , res : & mut Vec < T > , parse : impl Fn (& str) -> Result < T , response :: Error > ,) -> Result < bool , response :: Error > { line . clear () ; while reader . readline_str (line) ? != 0 { res . push (parse (line) ?) ; line . clear () ; } Ok (if reader . stopped_at () == Some (MessageKind :: Delimiter) { reader . reset (Protocol :: V2) ; false } else { true }) }
    };
}

parse_v2_section!()