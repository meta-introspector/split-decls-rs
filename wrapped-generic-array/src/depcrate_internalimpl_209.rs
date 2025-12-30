// Generated macro for impl_209 (impl)
macro_rules! Depcrate_internalimpl_209 {
() => {
// Module: crate::internal
// Provides: {"impl_209"}
// Dependencies: {}
impl < T , N : ArrayLength > ArrayConsumer < T , N > { # [doc = " Give ownership of the array to the consumer"] # [inline (always)] pub const fn new (array : GenericArray < T , N >) -> ArrayConsumer < T , N > { ArrayConsumer { array : ManuallyDrop :: new (array) , position : 0 , } } # [doc = " Creates an iterator and mutable reference to the internal position"] # [doc = " to keep track of consumed elements."] # [doc = ""] # [doc = " You MUST increment the position as you iterate to mark off consumed elements."] # [inline (always)] pub unsafe fn iter_position (& '_ mut self) -> (slice :: Iter < '_ , T > , & '_ mut usize) { (self . array . iter () , & mut self . position) } }
};
}
