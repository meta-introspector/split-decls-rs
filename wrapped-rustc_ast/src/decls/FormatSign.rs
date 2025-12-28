macro_rules! FormatSign {
    () => {
        # [derive (Copy , Clone , Encodable , Decodable , Debug , PartialEq , Eq)] pub enum FormatSign { # [doc = " The `+` flag."] Plus , # [doc = " The `-` flag."] Minus , }
    };
}

FormatSign!()