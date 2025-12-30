// Generated macro for Quoter (struct)
macro_rules! Depcrate_bytesQuoter {
() => {
// Module: crate::bytes
// Provides: {"Quoter"}
// Dependencies: {}
# [doc = " A more configurable interface to quote strings.  If you only want the default settings you can"] # [doc = " use the convenience functions [`try_quote`] and [`try_join`]."] # [doc = ""] # [doc = " The string equivalent is [`shlex::Quoter`]."] # [derive (Default , Debug , Clone)] pub struct Quoter { allow_nul : bool , }
};
}
