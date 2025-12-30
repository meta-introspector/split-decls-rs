// Generated macro for tests (module)
macro_rules! Depcrate_bantests {
() => {
// Module: crate::ban
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use regex_syntax :: Parser ; # [doc = " Returns true when the given pattern is detected to contain the given"] # [doc = " banned byte."] fn check (pattern : & str , byte : u8) -> bool { let hir = Parser :: new () . parse (pattern) . unwrap () ; super :: check (& hir , byte) . is_err () } # [test] fn various () { assert ! (check (r"\x00" , 0)) ; assert ! (check (r"a\x00" , 0)) ; assert ! (check (r"\x00b" , 0)) ; assert ! (check (r"a\x00b" , 0)) ; assert ! (check (r"\x00|ab" , 0)) ; assert ! (check (r"ab|\x00" , 0)) ; assert ! (check (r"\x00?" , 0)) ; assert ! (check (r"(\x00)" , 0)) ; assert ! (check (r"[\x00]" , 0)) ; assert ! (check (r"[^[^\x00]]" , 0)) ; assert ! (! check (r"[^\x00]" , 0)) ; assert ! (! check (r"[\x00a]" , 0)) ; } }
};
}
