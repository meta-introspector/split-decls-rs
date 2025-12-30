// Generated macro for impl_1681 (impl)
macro_rules! Depcrate_stringimpl_1681 {
() => {
// Module: crate::string
// Provides: {"impl_1681"}
// Dependencies: {}
impl < 'a > ParallelDrainRange < usize > for & 'a mut String { type Iter = Drain < 'a > ; type Item = char ; fn par_drain < R : RangeBounds < usize > > (self , range : R) -> Self :: Iter { Drain { range : simplify_range (range , self . len ()) , string : self , } } }
};
}
