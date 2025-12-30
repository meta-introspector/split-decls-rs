// Generated macro for impl_15 (impl)
macro_rules! Depcrate_split_producerimpl_15 {
() => {
// Module: crate::split_producer
// Provides: {"impl_15"}
// Dependencies: {}
impl < 'p , P , V > SplitProducer < 'p , P , V > where V : Fissile < P > + Send , { pub (super) fn new (data : V , separator : & 'p P) -> Self { SplitProducer { tail : data . length () , data , separator , } } }
};
}
