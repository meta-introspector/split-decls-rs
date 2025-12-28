macro_rules! deps {
    () => {
        AsciiParser!();
    };
}

macro_rules! DefaultCharAccumulator {
    () => {
        deps!();
        # [cfg (not (feature = "utf8"))] pub type DefaultCharAccumulator = AsciiParser ;
    };
}

DefaultCharAccumulator!();