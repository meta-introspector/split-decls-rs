macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! try_parse {
    () => {
        deps!();
        fn try_parse < T : FromStr + PartialEq + Default > (input : & BStr) -> Result < Option < T > , Error > { input . to_str () . ok () . and_then (| n | { n . parse () . ok () . map (| n | { if n == T :: default () && input [0] == b'-' { return Err (Error :: NegativeZero { input : input . into () }) ; } Ok (n) }) }) . transpose () }
    };
}

try_parse!()