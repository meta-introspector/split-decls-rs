// Generated macro for Flags (struct)
macro_rules! Depcrate_hir_translateFlags {
() => {
// Module: crate::hir::translate
// Provides: {"Flags"}
// Dependencies: {}
# [doc = " A translator's representation of a regular expression's flags at any given"] # [doc = " moment in time."] # [doc = ""] # [doc = " Each flag can be in one of three states: absent, present but disabled or"] # [doc = " present but enabled."] # [derive (Clone , Copy , Debug , Default)] struct Flags { case_insensitive : Option < bool > , multi_line : Option < bool > , dot_matches_new_line : Option < bool > , swap_greed : Option < bool > , unicode : Option < bool > , crlf : Option < bool > , }
};
}
