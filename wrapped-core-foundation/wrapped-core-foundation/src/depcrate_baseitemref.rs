// Generated macro for ItemRef (struct)
macro_rules! Depcrate_baseItemRef {
() => {
// Module: crate::base
// Provides: {"ItemRef"}
// Dependencies: {}
# [doc = " A reference to an element inside a container"] pub struct ItemRef < 'a , T : 'a > (ManuallyDrop < T > , PhantomData < & 'a T >) ;
};
}
