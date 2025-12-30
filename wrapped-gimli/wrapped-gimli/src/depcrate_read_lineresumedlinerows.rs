// Generated macro for ResumedLineRows (type)
macro_rules! Depcrate_read_lineResumedLineRows {
() => {
// Module: crate::read::line
// Provides: {"ResumedLineRows"}
// Dependencies: {}
type ResumedLineRows < 'program , R , Offset = < R as Reader > :: Offset > = LineRows < R , & 'program CompleteLineProgram < R , Offset > , Offset > ;
};
}
