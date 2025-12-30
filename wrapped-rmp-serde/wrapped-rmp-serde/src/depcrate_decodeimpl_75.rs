// Generated macro for impl_75 (impl)
macro_rules! Depcrate_decodeimpl_75 {
() => {
// Module: crate::decode
// Provides: {"impl_75"}
// Dependencies: {}
impl < R : Read > Deserializer < ReadReader < R > , DefaultConfig > { # [doc = " Constructs a new `Deserializer` by consuming the given reader."] # [inline] pub fn new (rd : R) -> Self { Self { rd : ReadReader :: new (rd) , _config : PhantomData , is_human_readable : DefaultConfig . is_human_readable () , marker : None , depth : 1024 , } } }
};
}
