// Generated macro for impl_212 (impl)
macro_rules! Depcrate_internalimpl_212 {
() => {
// Module: crate::internal
// Provides: {"impl_212"}
// Dependencies: {}
impl < 'a , T , N : ArrayLength > IntrusiveArrayConsumer < 'a , T , N > { # [doc = " Give exclusive access for the array to the consumer"] # [rustversion :: attr (since (1.83) , const)] # [inline (always)] pub fn new (array : & 'a mut ManuallyDrop < GenericArray < T , N > > ,) -> IntrusiveArrayConsumer < 'a , T , N > { IntrusiveArrayConsumer { array , position : 0 } } # [doc = " Creates an iterator and mutable reference to the internal position"] # [doc = " to keep track of consumed elements."] # [doc = ""] # [doc = " You MUST increment the position as you iterate to mark off consumed elements."] # [inline (always)] pub unsafe fn iter_position (& '_ mut self) -> (slice :: Iter < '_ , T > , & '_ mut usize) { (self . array . iter () , & mut self . position) } }
};
}
