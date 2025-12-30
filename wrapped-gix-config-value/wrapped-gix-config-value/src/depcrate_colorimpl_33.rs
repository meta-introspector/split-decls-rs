// Generated macro for impl_33 (impl)
macro_rules! Depcrate_colorimpl_33 {
() => {
// Module: crate::color
// Provides: {"impl_33"}
// Dependencies: {}
impl FromStr for Attribute { type Err = Error ; fn from_str (mut s : & str) -> Result < Self , Self :: Err > { let inverted = if let Some (rest) = s . strip_prefix ("no-") . or_else (| | s . strip_prefix ("no")) { s = rest ; true } else { false } ; match s { "reset" if ! inverted => Ok (Attribute :: RESET) , "reset" if inverted => Err (color_err (s)) , "bold" if ! inverted => Ok (Attribute :: BOLD) , "bold" if inverted => Ok (Attribute :: NO_BOLD) , "dim" if ! inverted => Ok (Attribute :: DIM) , "dim" if inverted => Ok (Attribute :: NO_DIM) , "ul" if ! inverted => Ok (Attribute :: UL) , "ul" if inverted => Ok (Attribute :: NO_UL) , "blink" if ! inverted => Ok (Attribute :: BLINK) , "blink" if inverted => Ok (Attribute :: NO_BLINK) , "reverse" if ! inverted => Ok (Attribute :: REVERSE) , "reverse" if inverted => Ok (Attribute :: NO_REVERSE) , "italic" if ! inverted => Ok (Attribute :: ITALIC) , "italic" if inverted => Ok (Attribute :: NO_ITALIC) , "strike" if ! inverted => Ok (Attribute :: STRIKE) , "strike" if inverted => Ok (Attribute :: NO_STRIKE) , _ => Err (color_err (s)) , } } }
};
}
