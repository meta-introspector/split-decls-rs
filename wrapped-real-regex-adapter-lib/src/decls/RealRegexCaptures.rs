macro_rules! RealRegexCaptures {
    () => {
        pub struct RealRegexCaptures < 't > { captures : regex :: Captures < 't > , }
    };
}

RealRegexCaptures!()