macro_rules! StabilityParser {
    () => {
        # [derive (Default)] pub (crate) struct StabilityParser { allowed_through_unstable_modules : Option < Symbol > , stability : Option < (Stability , Span) > , }
    };
}

StabilityParser!();