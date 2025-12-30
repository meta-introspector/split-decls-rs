// Generated macro for impl_666 (impl)
macro_rules! Depcrate_parser_sequenceimpl_666 {
() => {
// Module: crate::parser::sequence
// Provides: {"impl_666"}
// Dependencies: {}
impl < T , U > SequenceState < T , U > where U : Default , { unsafe fn unwrap_value (& mut self) -> T { match self . value . take () { Some (t) => t , None => core :: hint :: unreachable_unchecked () , } } }
};
}
