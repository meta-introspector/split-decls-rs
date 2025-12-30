// Generated macro for Quoter (struct)
macro_rules! DepcrateQuoter {
() => {
// Module: crate
// Provides: {"Quoter"}
// Dependencies: {}
# [doc = " A more configurable interface to quote strings.  If you only want the default settings you can"] # [doc = " use the convenience functions [`try_quote`] and [`try_join`]."] # [doc = ""] # [doc = " The bytes equivalent is [`bytes::Quoter`]."] # [derive (Default , Debug , Clone)] pub struct Quoter { inner : bytes :: Quoter , }
};
}
