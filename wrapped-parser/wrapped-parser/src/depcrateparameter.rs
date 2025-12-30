// Generated macro for Parameter (struct)
macro_rules! DepcrateParameter {
() => {
// Module: crate
// Provides: {"Parameter"}
// Dependencies: {}
# [doc = " A parameter of the form `{{0=Type:hint}}` in a format string."] # [derive (Clone , Debug , Eq , PartialEq)] pub struct Parameter { # [doc = " The argument index to display at this position."] pub index : usize , # [doc = " The type of the argument to display, e.g. '=u8', '=bool'."] pub ty : Type , # [doc = " The display hint, e.g. ':x', ':b', ':a'."] pub hint : Option < DisplayHint > , }
};
}
