macro_rules! HexFloatParseError {
    () => {
        # [derive (Copy , Clone , Debug)] pub struct HexFloatParseError (& 'static str) ;
    };
}

HexFloatParseError!();