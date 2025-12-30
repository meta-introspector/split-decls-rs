// Generated macro for is_wsp (function)
macro_rules! Depcrate_grammaris_wsp {
() => {
// Module: crate::grammar
// Provides: {"is_wsp"}
// Dependencies: {}
# [doc = " Does the provided byte match the \"WSP\" ABNF production from Section 3?"] # [doc = ""] # [doc = " > The common ABNF production WSP is congruent with \"blank\";"] # [doc = " > a new production W is used for \"whitespace\""] pub (crate) fn is_wsp (char : u8) -> bool { matches ! (char , CHAR_HT | CHAR_SP) }
};
}
