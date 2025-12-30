// Generated macro for impl_38 (impl)
macro_rules! Depcrate_msgimpl_38 {
() => {
// Module: crate::msg
// Provides: {"impl_38"}
// Dependencies: {}
impl Iterator for CommandMessages { type Item = CargoResult < Message > ; # [inline] fn next (& mut self) -> Option < CargoResult < Message > > { match self . next_msg () { Ok (Some (x)) => Some (Ok (x)) , Ok (None) => None , Err (e) => Some (Err (e)) , } } }
};
}
