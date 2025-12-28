macro_rules! deps {
    () => {
        Parser!();
    };
}

macro_rules! Utf8Parser {
    () => {
        deps!();
        # [doc = " Allow parsing UTF-8"] # [cfg (feature = "utf8")] # [derive (Default , Clone , Debug , PartialEq , Eq)] pub struct Utf8Parser { utf8_parser : utf8 :: Parser , }
    };
}

Utf8Parser!();