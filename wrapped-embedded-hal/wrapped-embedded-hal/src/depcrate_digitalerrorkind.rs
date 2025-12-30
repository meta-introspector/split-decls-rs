// Generated macro for ErrorKind (enum)
macro_rules! Depcrate_digitalErrorKind {
() => {
// Module: crate::digital
// Provides: {"ErrorKind"}
// Dependencies: {}
# [doc = " Error kind."] # [doc = ""] # [doc = " This represents a common set of operation errors. HAL implementations are"] # [doc = " free to define more specific or additional error types. However, by providing"] # [doc = " a mapping to these common errors, generic code can still react to them."] # [derive (Debug , Copy , Clone , Eq , PartialEq , Ord , PartialOrd , Hash)] # [cfg_attr (feature = "defmt-03" , derive (defmt :: Format))] # [non_exhaustive] pub enum ErrorKind { # [doc = " A different error occurred. The original error may contain more information."] Other , }
};
}
