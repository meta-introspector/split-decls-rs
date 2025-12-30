// Generated macro for check_disjoint (function)
macro_rules! Depcrate_text_editcheck_disjoint {
() => {
// Module: crate::text_edit
// Provides: {"check_disjoint"}
// Dependencies: {}
fn check_disjoint < 'a , I > (indels : & mut I) -> bool where I : std :: iter :: Iterator < Item = & 'a Indel > + Clone , { indels . clone () . zip (indels . skip (1)) . all (| (l , r) | l . delete . end () <= r . delete . start () || l == r) }
};
}
