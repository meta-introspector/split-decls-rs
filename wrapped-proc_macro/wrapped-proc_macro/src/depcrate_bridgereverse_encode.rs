// Generated macro for reverse_encode (macro)
macro_rules! Depcrate_bridgereverse_encode {
() => {
// Module: crate::bridge
// Provides: {"reverse_encode"}
// Dependencies: {}
macro_rules ! reverse_encode { ($ writer : ident ;) => { } ; ($ writer : ident ; $ first : ident $ (, $ rest : ident) *) => { reverse_encode ! ($ writer ; $ ($ rest) ,*) ; $ first . encode (& mut $ writer , & mut ()) ; } }
};
}
