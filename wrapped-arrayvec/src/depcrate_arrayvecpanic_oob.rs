// Generated macro for panic_oob (macro)
macro_rules! Depcrate_arrayvecpanic_oob {
() => {
// Module: crate::arrayvec
// Provides: {"panic_oob"}
// Dependencies: {}
macro_rules ! panic_oob { ($ method_name : expr , $ index : expr , $ len : expr) => { panic ! (concat ! ("ArrayVec::" , $ method_name , ": index {} is out of bounds in vector of length {}") , $ index , $ len) } ; }
};
}
