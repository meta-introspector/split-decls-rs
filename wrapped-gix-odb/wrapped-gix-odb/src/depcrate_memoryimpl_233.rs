// Generated macro for impl_233 (impl)
macro_rules! Depcrate_memoryimpl_233 {
() => {
// Module: crate::memory
// Provides: {"impl_233"}
// Dependencies: {}
impl Proxy < Cache < crate :: store :: Handle < Rc < crate :: Store > > > > { # [doc = " Create an entirely new instance, but with the in-memory objects moving between them."] pub fn into_arc (self) -> std :: io :: Result < Proxy < Cache < crate :: store :: Handle < Arc < crate :: Store > > > > > { Ok (Proxy { inner : self . inner . into_arc () ? , object_hash : self . object_hash , memory : self . memory , }) } }
};
}
