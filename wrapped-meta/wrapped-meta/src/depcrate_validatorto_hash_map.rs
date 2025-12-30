// Generated macro for to_hash_map (function)
macro_rules! Depcrate_validatorto_hash_map {
() => {
// Module: crate::validator
// Provides: {"to_hash_map"}
// Dependencies: {}
fn to_hash_map < 'a , 'i : 'a > (rules : & 'a [ParserRule < 'i >]) -> HashMap < String , & 'a ParserNode < 'i > > { rules . iter () . map (| r | (r . name . clone () , & r . node)) . collect () }
};
}
