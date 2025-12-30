// Generated macro for SplitTerminatorProducer (struct)
macro_rules! Depcrate_strSplitTerminatorProducer {
() => {
// Module: crate::str
// Provides: {"SplitTerminatorProducer"}
// Dependencies: {}
struct SplitTerminatorProducer < 'ch , 'sep , P : Pattern > { splitter : SplitProducer < 'sep , P , & 'ch str > , skip_last : bool , }
};
}
