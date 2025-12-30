// Generated macro for ValueFormatter (type)
macro_rules! DepcrateValueFormatter {
() => {
// Module: crate
// Provides: {"ValueFormatter"}
// Dependencies: {}
# [doc = " Type alias for closures which can be used as value formatters."] pub type ValueFormatter = dyn Fn (& Value , & mut String) -> Result < () > ;
};
}
