macro_rules! deps {
    () => {
        ParsedRule!();
        RawPattern!();
    };
}

macro_rules! SsrRule {
    () => {
        deps!();
        # [derive (Debug)] pub struct SsrRule { # [doc = " A structured pattern that we're searching for."] pattern : parsing :: RawPattern , # [doc = " What we'll replace it with."] template : parsing :: RawPattern , parsed_rules : Vec < parsing :: ParsedRule > , }
    };
}

SsrRule!()