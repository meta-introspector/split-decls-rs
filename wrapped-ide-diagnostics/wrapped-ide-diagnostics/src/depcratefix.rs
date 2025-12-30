// Generated macro for fix (function)
macro_rules! Depcratefix {
() => {
// Module: crate
// Provides: {"fix"}
// Dependencies: {}
fn fix (id : & 'static str , label : & str , source_change : SourceChange , target : TextRange) -> Assist { let mut res = unresolved_fix (id , label , target) ; res . source_change = Some (source_change) ; res }
};
}
