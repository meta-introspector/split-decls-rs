macro_rules! ParseError {
    () => {
        # [doc = " This enum type contains errors that can occur when using a DotParser. In principle, those"] # [doc = " errors should never occur, as all parsing errors should be caught by DotParser::parse."] # [doc = " Therefore, if such error occurs, it is a bug, probably a missing feature."] # [derive (Debug , Clone)] pub enum ParseError < 'a > { # [doc = " This variant represents the case where we expect one of several rules, but we actually find"] # [doc = " another."] ExpectRule { # [doc = " The list of accepted `Rule`s."] expect : Vec < Rule > , # [doc = " The `Rule` actually found."] found : Rule , } , # [doc = " This variant represents the case where we expect a `Pair` but none is present."] MissingPair { # [doc = " The parent pair, i.e. the one that terminates too early."] parent : Pair < 'a , Rule > , # [doc = " The missing pair should have one of the `Rule` in `expect`."] expect : Vec < Rule > , } , }
    };
}

ParseError!()