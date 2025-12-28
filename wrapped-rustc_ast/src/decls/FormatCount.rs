macro_rules! deps {
    () => {
        FormatArgPosition!();
    };
}

macro_rules! FormatCount {
    () => {
        deps!();
        # [derive (Clone , Encodable , Decodable , Debug , PartialEq , Eq)] pub enum FormatCount { # [doc = " `{:5}` or `{:.5}`"] Literal (u16) , # [doc = " `{:.*}`, `{:.5$}`, or `{:a$}`, etc."] Argument (FormatArgPosition) , }
    };
}

FormatCount!();