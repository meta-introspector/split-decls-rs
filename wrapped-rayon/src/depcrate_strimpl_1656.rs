// Generated macro for impl_1656 (impl)
macro_rules! Depcrate_strimpl_1656 {
() => {
// Module: crate::str
// Provides: {"impl_1656"}
// Dependencies: {}
impl < 'ch , 'sep , P : Pattern + 'sep > SplitTerminatorProducer < 'ch , 'sep , P > { fn new (chars : & 'ch str , terminator : & 'sep P) -> Self { SplitTerminatorProducer { splitter : SplitProducer :: new (chars , terminator) , skip_last : chars . is_empty () || terminator . is_suffix_of (chars) , } } }
};
}
