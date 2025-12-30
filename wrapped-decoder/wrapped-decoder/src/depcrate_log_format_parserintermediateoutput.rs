// Generated macro for IntermediateOutput (enum)
macro_rules! Depcrate_log_format_parserIntermediateOutput {
() => {
// Module: crate::log::format::parser
// Provides: {"IntermediateOutput"}
// Dependencies: {}
# [derive (Debug , PartialEq , Clone)] enum IntermediateOutput { Metadata (LogMetadata) , WidthAndAlignment ((usize , Padding , Option < Alignment >)) , Color (LogColor) , Style (colored :: Styles) , NestedLogSegment (LogSegment) , }
};
}
