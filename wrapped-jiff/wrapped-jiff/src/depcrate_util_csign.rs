// Generated macro for Sign (enum)
macro_rules! Depcrate_util_cSign {
() => {
// Module: crate::util::c
// Provides: {"Sign"}
// Dependencies: {}
# [doc = " A representation of a numeric sign."] # [doc = ""] # [doc = " Its `Display` impl emits the ASCII minus sign, `-` when this"] # [doc = " is negative. It emits the empty string in all other cases."] # [derive (Clone , Copy , Debug , Default , Eq , Hash , PartialEq , PartialOrd , Ord ,)] # [repr (i8)] pub (crate) enum Sign { # [default] Zero = 0 , Positive = 1 , Negative = - 1 , }
};
}
