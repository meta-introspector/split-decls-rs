macro_rules! FormatArgPositionKind {
    () => {
        # [derive (Copy , Clone , Encodable , Decodable , Debug , PartialEq , Eq)] pub enum FormatArgPositionKind { # [doc = " `{}` or `{:.*}`"] Implicit , # [doc = " `{1}` or `{:1$}` or `{:.1$}`"] Number , # [doc = " `{a}` or `{:a$}` or `{:.a$}`"] Named , }
    };
}

FormatArgPositionKind!();