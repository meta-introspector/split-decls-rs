// Generated macro for ItemMutRef (struct)
macro_rules! Depcrate_baseItemMutRef {
() => {
// Module: crate::base
// Provides: {"ItemMutRef"}
// Dependencies: {}
# [doc = " A reference to a mutable element inside a container"] pub struct ItemMutRef < 'a , T : 'a > (ManuallyDrop < T > , PhantomData < & 'a T >) ;
};
}
