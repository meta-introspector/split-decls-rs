// Generated macro for StandardImpl (struct)
macro_rules! Depcrate_standardStandardImpl {
() => {
// Module: crate::standard
// Provides: {"StandardImpl"}
// Dependencies: {}
# [doc = " The actual implementation of the standard printer. This couples together"] # [doc = " the searcher, the sink implementation and information about the match."] # [doc = ""] # [doc = " A StandardImpl is initialized every time a match or a contextual line is"] # [doc = " reported."] # [derive (Debug)] struct StandardImpl < 'a , M : Matcher , W > { searcher : & 'a Searcher , sink : & 'a StandardSink < 'a , 'a , M , W > , sunk : Sunk < 'a > , # [doc = " Set to true if and only if we are writing a match with color."] in_color_match : Cell < bool > , }
};
}
