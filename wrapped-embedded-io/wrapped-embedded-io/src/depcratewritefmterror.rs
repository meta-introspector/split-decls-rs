// Generated macro for WriteFmtError (enum)
macro_rules! DepcrateWriteFmtError {
() => {
// Module: crate
// Provides: {"WriteFmtError"}
// Dependencies: {}
# [doc = " Error returned by [`Write::write_fmt`]"] # [derive (Debug , Copy , Clone , Eq , PartialEq)] # [cfg_attr (feature = "defmt" , derive (defmt :: Format))] pub enum WriteFmtError < E > { # [doc = " An error was encountered while formatting."] FmtError , # [doc = " Error returned by the inner Write."] Other (E) , }
};
}
