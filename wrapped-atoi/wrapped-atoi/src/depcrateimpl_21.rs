// Generated macro for impl_21 (impl)
macro_rules! Depcrateimpl_21 {
() => {
// Module: crate
// Provides: {"impl_21"}
// Dependencies: {}
impl Sign { # [doc = " Trys to convert an ascii character into a `Sign`"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use atoi::Sign;"] # [doc = " assert_eq!(Some(Sign::Plus), Sign::try_from(b'+'));"] # [doc = " assert_eq!(Some(Sign::Minus), Sign::try_from(b'-'));"] # [doc = " assert_eq!(None, Sign::try_from(b'1'));"] # [doc = " ```"] pub fn try_from (byte : u8) -> Option < Sign > { match byte { b'+' => Some (Sign :: Plus) , b'-' => Some (Sign :: Minus) , _ => None , } } # [doc = " Returns either `+1` or `-1`"] pub fn signum < I > (self) -> I where I : Signed , { match self { Sign :: Plus => I :: one () , Sign :: Minus => - I :: one () , } } }
};
}
