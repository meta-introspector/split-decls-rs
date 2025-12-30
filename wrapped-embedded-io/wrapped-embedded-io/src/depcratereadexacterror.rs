// Generated macro for ReadExactError (enum)
macro_rules! DepcrateReadExactError {
() => {
// Module: crate
// Provides: {"ReadExactError"}
// Dependencies: {}
# [doc = " Error returned by [`Read::read_exact`]"] # [derive (Debug , Copy , Clone , Eq , PartialEq)] # [cfg_attr (feature = "defmt" , derive (defmt :: Format))] pub enum ReadExactError < E > { # [doc = " An EOF error was encountered before reading the exact amount of requested bytes."] UnexpectedEof , # [doc = " Error returned by the inner Read."] Other (E) , }
};
}
