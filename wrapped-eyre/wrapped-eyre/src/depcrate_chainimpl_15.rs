// Generated macro for impl_15 (impl)
macro_rules! Depcrate_chainimpl_15 {
() => {
// Module: crate::chain
// Provides: {"impl_15"}
// Dependencies: {}
impl < 'a > Chain < 'a > { # [doc = " Construct an iterator over a chain of errors via the `source` method"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " use std::error::Error;"] # [doc = " use std::fmt::{self, Write};"] # [doc = " use eyre::Chain;"] # [doc = " use indenter::indented;"] # [doc = ""] # [doc = " fn report(error: &(dyn Error + 'static), f: &mut fmt::Formatter<'_>) -> fmt::Result {"] # [doc = "     let mut errors = Chain::new(error).enumerate();"] # [doc = "     for (i, error) in errors {"] # [doc = "         writeln!(f)?;"] # [doc = "         write!(indented(f).ind(i), \"{}\", error)?;"] # [doc = "     }"] # [doc = ""] # [doc = "     Ok(())"] # [doc = " }"] # [doc = " ```"] pub fn new (head : & 'a (dyn StdError + 'static)) -> Self { Chain { state : ChainState :: Linked { next : Some (head) } , } } }
};
}
