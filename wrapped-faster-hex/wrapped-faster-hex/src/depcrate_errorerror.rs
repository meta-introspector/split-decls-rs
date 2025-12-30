// Generated macro for Error (enum)
macro_rules! Depcrate_errorError {
() => {
// Module: crate::error
// Provides: {"Error"}
// Dependencies: {}
# [derive (Clone , Copy , PartialEq)] # [cfg_attr (feature = "defmt-03" , derive (defmt :: Format))] pub enum Error { InvalidChar , InvalidLength (usize) , Overflow , }
};
}
