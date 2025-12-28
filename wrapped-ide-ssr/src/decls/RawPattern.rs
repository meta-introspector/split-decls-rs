macro_rules! deps {
    () => {
        PatternElement!();
    };
}

macro_rules! RawPattern {
    () => {
        deps!();
        # [derive (Debug)] pub (crate) struct RawPattern { tokens : Vec < PatternElement > , }
    };
}

RawPattern!()