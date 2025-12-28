macro_rules! deps {
    () => {
        Match!();
        Input!();
        DebugByte!();
    };
}

macro_rules! LookMatcher {
    () => {
        deps!();
        # [doc = " A matcher for look-around assertions."] # [doc = ""] # [doc = " This matcher permits configuring aspects of how look-around assertions are"] # [doc = " matched."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " A `LookMatcher` can change the line terminator used for matching multi-line"] # [doc = " anchors such as `(?m:^)` and `(?m:$)`."] # [doc = ""] # [doc = " ```"] # [doc = " use regex_automata::{"] # [doc = "     nfa::thompson::{self, pikevm::PikeVM},"] # [doc = "     util::look::LookMatcher,"] # [doc = "     Match, Input,"] # [doc = " };"] # [doc = ""] # [doc = " let mut lookm = LookMatcher::new();"] # [doc = " lookm.set_line_terminator(b'\\x00');"] # [doc = ""] # [doc = " let re = PikeVM::builder()"] # [doc = "     .thompson(thompson::Config::new().look_matcher(lookm))"] # [doc = "     .build(r\"(?m)^[a-z]+$\")?;"] # [doc = " let mut cache = re.create_cache();"] # [doc = ""] # [doc = " // Multi-line assertions now use NUL as a terminator."] # [doc = " assert_eq!("] # [doc = "     Some(Match::must(0, 1..4)),"] # [doc = "     re.find(&mut cache, b\"\\x00abc\\x00\"),"] # [doc = " );"] # [doc = " // ... and \\n is no longer recognized as a terminator."] # [doc = " assert_eq!("] # [doc = "     None,"] # [doc = "     re.find(&mut cache, b\"\\nabc\\n\"),"] # [doc = " );"] # [doc = ""] # [doc = " # Ok::<(), Box<dyn std::error::Error>>(())"] # [doc = " ```"] # [derive (Clone , Debug)] pub struct LookMatcher { lineterm : DebugByte , }
    };
}

LookMatcher!()