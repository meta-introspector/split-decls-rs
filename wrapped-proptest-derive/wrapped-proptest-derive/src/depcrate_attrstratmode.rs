// Generated macro for StratMode (enum)
macro_rules! Depcrate_attrStratMode {
() => {
// Module: crate::attr
// Provides: {"StratMode"}
// Dependencies: {}
# [doc = " The mode for the associated item `Strategy` to use."] # [derive (Clone)] pub enum StratMode { # [doc = " This means that no explicit strategy was specified"] # [doc = " and that we thus should use `Arbitrary` for whatever"] # [doc = " it is that needs a strategy."] Arbitrary , # [doc = " This means that an explicit value has been provided."] # [doc = " The result of this is to use a strategy that always"] # [doc = " returns the given value."] Value (Expr) , # [doc = " This means that an explicit strategy has been provided."] # [doc = " This strategy will be used to generate whatever it"] # [doc = " is that the attribute was set on."] Strategy (Expr) , # [doc = " This means that an explicit *regex* strategy has been provided."] # [doc = " We don't reuse `Strategy(..)` so that we can produce better and"] # [doc = " more tailored error messages."] Regex (Expr) , }
};
}
