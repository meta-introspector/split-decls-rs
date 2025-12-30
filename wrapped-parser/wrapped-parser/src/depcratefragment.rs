// Generated macro for Fragment (enum)
macro_rules! DepcrateFragment {
() => {
// Module: crate
// Provides: {"Fragment"}
// Dependencies: {}
# [doc = " A part of a format string."] # [derive (Clone , Debug , Eq , PartialEq)] pub enum Fragment < 'f > { # [doc = " A literal string (eg. `\"literal \"` in `\"literal {:?}\"`)."] Literal (Cow < 'f , str >) , # [doc = " A format parameter."] Parameter (Parameter) , }
};
}
