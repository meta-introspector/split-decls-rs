// Generated macro for Inputs (type)
macro_rules! DepcrateInputs {
() => {
// Module: crate
// Provides: {"Inputs"}
// Dependencies: {}
# [doc = " A map of input names and values."] # [doc = " The names include their `$` prefix."] pub type Inputs < 'a > = HashMap < & 'a str , Value < 'a > > ;
};
}
