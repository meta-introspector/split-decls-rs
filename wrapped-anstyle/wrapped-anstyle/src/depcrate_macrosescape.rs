// Generated macro for escape (macro)
macro_rules! Depcrate_macrosescape {
() => {
// Module: crate::macros
// Provides: {"escape"}
// Dependencies: {}
macro_rules ! escape { ($ ($ inner : expr) ,*) => { concat ! ("\x1B[" , $ ($ inner) ,*, "m") } ; }
};
}
