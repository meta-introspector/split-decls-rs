// Generated macro for define_signal_enum (macro)
macro_rules! Depcratedefine_signal_enum {
() => {
// Module: crate
// Provides: {"define_signal_enum"}
// Dependencies: {}
macro_rules ! define_signal_enum { ($ (# [$ outer : meta]) * pub enum Signal { $ ($ (# [$ inner : meta]) * $ name : ident = $ value : ident ,) * }) => { $ (# [$ outer]) * # [derive (Copy , Clone , Debug , Eq , PartialEq , PartialOrd , Ord , Hash)] # [repr (i32)] pub enum Signal { $ ($ (# [$ inner]) * $ name = signum ::$ value ,) * } impl Signal { # [doc = " Returns the signal number."] fn number (self) -> std :: os :: raw :: c_int { match self { $ (Signal ::$ name => signum ::$ value ,) * } } # [doc = " Parse a signal from its number."] # [cfg (unix)] fn from_number (number : std :: os :: raw :: c_int) -> Option < Self > { match number { $ (signum ::$ value => Some (Signal ::$ name) ,) * _ => None , } } } } }
};
}
