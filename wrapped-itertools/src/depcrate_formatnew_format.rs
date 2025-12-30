// Generated macro for new_format (function)
macro_rules! Depcrate_formatnew_format {
() => {
// Module: crate::format
// Provides: {"new_format"}
// Dependencies: {}
pub fn new_format < I , F > (iter : I , separator : & str , f : F) -> FormatWith < '_ , I , F > where I : Iterator , F : FnMut (I :: Item , & mut dyn FnMut (& dyn fmt :: Display) -> fmt :: Result) -> fmt :: Result , { FormatWith { sep : separator , inner : Cell :: new (Some ((iter , f))) , } }
};
}
