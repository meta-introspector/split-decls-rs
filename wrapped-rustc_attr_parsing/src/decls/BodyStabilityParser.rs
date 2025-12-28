macro_rules! BodyStabilityParser {
    () => {
        # [derive (Default)] pub (crate) struct BodyStabilityParser { stability : Option < (DefaultBodyStability , Span) > , }
    };
}

BodyStabilityParser!()