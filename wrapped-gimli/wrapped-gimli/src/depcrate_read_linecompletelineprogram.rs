// Generated macro for CompleteLineProgram (struct)
macro_rules! Depcrate_read_lineCompleteLineProgram {
() => {
// Module: crate::read::line
// Provides: {"CompleteLineProgram"}
// Dependencies: {}
# [doc = " A line number program that has previously been run to completion."] # [derive (Clone , Debug , Eq , PartialEq)] pub struct CompleteLineProgram < R , Offset = < R as Reader > :: Offset > where R : Reader < Offset = Offset > , Offset : ReaderOffset , { header : LineProgramHeader < R , Offset > , }
};
}
