// Generated macro for QuoteError (enum)
macro_rules! DepcrateQuoteError {
() => {
// Module: crate
// Provides: {"QuoteError"}
// Dependencies: {}
# [doc = " Errors from [`Quoter::quote`], [`Quoter::join`], etc. (and their [`bytes`] counterparts)."] # [doc = ""] # [doc = " By default, the only error that can be returned is [`QuoteError::Nul`].  If you call"] # [doc = " `allow_nul(true)`, then no errors can be returned at all.  Any error variants added in the"] # [doc = " future will not be enabled by default; they will be enabled through corresponding non-default"] # [doc = " [`Quoter`] options."] # [doc = ""] # [doc = " ...In theory.  In the unlikely event that additional classes of inputs are discovered that,"] # [doc = " like nul bytes, are fundamentally unsafe to quote even for non-interactive shells, the risk"] # [doc = " will be mitigated by adding corresponding [`QuoteError`] variants that *are* enabled by"] # [doc = " default."] # [non_exhaustive] # [derive (Debug , Clone , Copy , PartialEq , Eq , PartialOrd , Ord , Hash)] pub enum QuoteError { # [doc = " The input contained a nul byte.  In most cases, shells fundamentally [cannot handle strings"] # [doc = " containing nul bytes](quoting_warning#nul-bytes), no matter how they are quoted.  But if"] # [doc = " you're sure you can handle nul bytes, you can call `allow_nul(true)` on the `Quoter` to let"] # [doc = " them pass through."] Nul , }
};
}
