// Generated macro for coalesce_indels (function)
macro_rules! Depcrate_text_editcoalesce_indels {
() => {
// Module: crate::text_edit
// Provides: {"coalesce_indels"}
// Dependencies: {}
fn coalesce_indels (indels : Vec < Indel >) -> Vec < Indel > { indels . into_iter () . coalesce (| mut a , b | { if a . delete . end () == b . delete . start () { a . insert . push_str (& b . insert) ; a . delete = TextRange :: new (a . delete . start () , b . delete . end ()) ; Ok (a) } else { Err ((a , b)) } }) . collect_vec () }
};
}
