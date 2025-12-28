macro_rules! Utf8Parser {
    () => {
        # [derive (Default , Clone , Debug , PartialEq , Eq)] pub (crate) struct Utf8Parser { utf8_parser : utf8parse :: Parser , }
    };
}

Utf8Parser!();