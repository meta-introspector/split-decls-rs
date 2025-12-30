// Generated macro for Pat (enum)
macro_rules! Depcrate_check_proc_macroPat {
() => {
// Module: crate::check_proc_macro
// Provides: {"Pat"}
// Dependencies: {}
# [doc = " The search pattern to look for. Used by `span_matches_pat`"] # [derive (Clone)] pub enum Pat { # [doc = " A single string."] Str (& 'static str) , # [doc = " Any of the given strings."] MultiStr (& 'static [& 'static str]) , # [doc = " Any of the given strings."] OwnedMultiStr (Vec < String >) , # [doc = " The string representation of the symbol."] Sym (Symbol) , # [doc = " Any decimal or hexadecimal digit depending on the location."] Num , }
};
}
