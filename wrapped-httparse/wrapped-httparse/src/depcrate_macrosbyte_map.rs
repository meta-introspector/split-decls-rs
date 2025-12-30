// Generated macro for byte_map (macro)
macro_rules! Depcrate_macrosbyte_map {
() => {
// Module: crate::macros
// Provides: {"byte_map"}
// Dependencies: {}
macro_rules ! byte_map { ($ ($ p : pat_param) |+) => { { const fn make_map () -> [bool ; 256] { let mut ret = [false ; 256] ; let mut i = 0 ; while i < 256 { ret [i] = matches ! (i as u8 , $ ($ p) |+) ; i += 1 ; } ret } make_map () } } }
};
}
