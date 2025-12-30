// Generated macro for human_output (function)
macro_rules! Depcrate_pack_indexhuman_output {
() => {
// Module: crate::pack::index
// Provides: {"human_output"}
// Dependencies: {}
fn human_output (mut out : impl io :: Write , res : pack :: bundle :: write :: Outcome) -> io :: Result < () > { writeln ! (& mut out , "index: {}" , res . index . index_hash) ? ; writeln ! (& mut out , "pack: {}" , res . index . data_hash) }
};
}
