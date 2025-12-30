// Generated macro for impl_655 (impl)
macro_rules! Depcrate_options_shapeimpl_655 {
() => {
// Module: crate::options::shape
// Provides: {"impl_655"}
// Dependencies: {}
impl DataShape { fn new (prefix : & 'static str) -> Self { DataShape { prefix , .. Default :: default () } } fn set_word (& mut self , word : & str) -> Result < () > { match word . trim_start_matches (self . prefix) { "newtype" => { self . newtype = true ; Ok (()) } "named" => { self . named = true ; Ok (()) } "tuple" => { self . tuple = true ; Ok (()) } "unit" => { self . unit = true ; Ok (()) } "any" => { self . any = true ; Ok (()) } _ => Err (Error :: unknown_value (word)) , } } }
};
}
