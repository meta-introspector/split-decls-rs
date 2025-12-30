// Generated macro for ConstPointer (struct)
macro_rules! Depcrate_ptrConstPointer {
() => {
// Module: crate::ptr
// Provides: {"ConstPointer"}
// Dependencies: {}
# [derive (Debug)] pub (crate) struct ConstPointer < 'a , T > { ptr : * const T , _lifetime : PhantomData < & 'a T > , }
};
}
