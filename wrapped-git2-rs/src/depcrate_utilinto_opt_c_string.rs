// Generated macro for into_opt_c_string (function)
macro_rules! Depcrate_utilinto_opt_c_string {
() => {
// Module: crate::util
// Provides: {"into_opt_c_string"}
// Dependencies: {}
pub fn into_opt_c_string < S > (opt_s : Option < S >) -> Result < Option < CString > , Error > where S : IntoCString , { match opt_s { None => Ok (None) , Some (s) => Ok (Some (s . into_c_string () ?)) , } }
};
}
