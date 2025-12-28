macro_rules! FormatAlignment {
    () => {
        # [derive (Copy , Clone , Encodable , Decodable , Debug , PartialEq , Eq)] pub enum FormatAlignment { # [doc = " `{:<}`"] Left , # [doc = " `{:>}`"] Right , # [doc = " `{:^}`"] Center , }
    };
}

FormatAlignment!();