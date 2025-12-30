// Generated macro for impl_329 (impl)
macro_rules! Depcrate_builder_styled_strimpl_329 {
() => {
// Module: crate::builder::styled_str
// Provides: {"impl_329"}
// Dependencies: {}
impl std :: fmt :: Write for StyledStr { # [inline] fn write_str (& mut self , s : & str) -> Result < () , std :: fmt :: Error > { self . 0 . push_str (s) ; Ok (()) } # [inline] fn write_char (& mut self , c : char) -> Result < () , std :: fmt :: Error > { self . 0 . push (c) ; Ok (()) } }
};
}
