macro_rules! deps {
    () => {
        ParsedRule!();
    };
}

macro_rules! SsrPattern {
    () => {
        deps!();
        # [derive (Debug)] pub struct SsrPattern { parsed_rules : Vec < parsing :: ParsedRule > , }
    };
}

SsrPattern!();