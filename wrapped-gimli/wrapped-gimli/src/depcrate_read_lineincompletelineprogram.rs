// Generated macro for IncompleteLineProgram (struct)
macro_rules! Depcrate_read_lineIncompleteLineProgram {
() => {
// Module: crate::read::line
// Provides: {"IncompleteLineProgram"}
// Dependencies: {}
# [doc = " A line number program that has not been run to completion."] # [derive (Clone , Debug , Eq , PartialEq)] pub struct IncompleteLineProgram < R , Offset = < R as Reader > :: Offset > where R : Reader < Offset = Offset > , Offset : ReaderOffset , { header : LineProgramHeader < R , Offset > , }
};
}
