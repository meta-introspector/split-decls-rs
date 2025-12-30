// Generated macro for impl_1098 (impl)
macro_rules! Depcrate_iter_sumimpl_1098 {
() => {
// Module: crate::iter::sum
// Provides: {"impl_1098"}
// Dependencies: {}
impl < S , T > Consumer < T > for SumConsumer < S > where S : Send + Sum < T > + Sum , { type Folder = SumFolder < S > ; type Reducer = Self ; type Result = S ; fn split_at (self , _index : usize) -> (Self , Self , Self) { (SumConsumer :: new () , SumConsumer :: new () , SumConsumer :: new ()) } fn into_folder (self) -> Self :: Folder { SumFolder { sum : iter :: empty :: < T > () . sum () , } } fn full (& self) -> bool { false } }
};
}
