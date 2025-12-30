// Generated macro for AutotagOption (enum)
macro_rules! DepcrateAutotagOption {
() => {
// Module: crate
// Provides: {"AutotagOption"}
// Dependencies: {}
# [doc = " Automatic tag following options."] # [derive (Copy , Clone , Debug , PartialEq , Eq)] pub enum AutotagOption { # [doc = " Use the setting from the remote's configuration"] Unspecified , # [doc = " Ask the server for tags pointing to objects we're already downloading"] Auto , # [doc = " Don't ask for any tags beyond the refspecs"] None , # [doc = " Ask for all the tags"] All , }
};
}
