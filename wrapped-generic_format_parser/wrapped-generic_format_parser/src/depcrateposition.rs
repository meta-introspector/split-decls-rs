// Generated macro for Position (enum)
macro_rules! DepcratePosition {
() => {
// Module: crate
// Provides: {"Position"}
// Dependencies: {}
# [doc = " Enum describing where an argument for a format can be located."] # [derive (Copy , Clone , Debug , PartialEq)] pub enum Position < 'a > { # [doc = " The argument is implied to be located at an index"] ArgumentImplicitlyIs (usize) , # [doc = " The argument is located at a specific index given in the format,"] ArgumentIs (usize) , # [doc = " The argument has a name."] ArgumentNamed (& 'a str) , }
};
}
