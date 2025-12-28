macro_rules! ConstStabilityParser {
    () => {
        # [derive (Default)] pub (crate) struct ConstStabilityParser { promotable : bool , stability : Option < (PartialConstStability , Span) > , }
    };
}

ConstStabilityParser!();