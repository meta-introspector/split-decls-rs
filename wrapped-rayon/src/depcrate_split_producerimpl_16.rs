// Generated macro for impl_16 (impl)
macro_rules! Depcrate_split_producerimpl_16 {
() => {
// Module: crate::split_producer
// Provides: {"impl_16"}
// Dependencies: {}
impl < 'p , P , V > SplitInclusiveProducer < 'p , P , V > where V : Fissile < P > + Send , { pub (super) fn new_incl (data : V , separator : & 'p P) -> Self { SplitProducer { tail : data . length () , data , separator , } } }
};
}
