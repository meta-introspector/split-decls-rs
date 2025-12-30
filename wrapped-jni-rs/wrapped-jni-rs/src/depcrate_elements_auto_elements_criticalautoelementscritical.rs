// Generated macro for AutoElementsCritical (struct)
macro_rules! Depcrate_elements_auto_elements_criticalAutoElementsCritical {
() => {
// Module: crate::elements::auto_elements_critical
// Provides: {"AutoElementsCritical"}
// Dependencies: {}
# [doc = " Auto-release wrapper for a mutable pointer to the elements of a [`JPrimitiveArray`]"] # [doc = " (such as [`JByteArray`])"] # [doc = ""] # [doc = " This type is used to wrap pointers returned by `GetPrimitiveArrayCritical`"] # [doc = " and ensure the pointer is released via `ReleasePrimitiveArrayCritical` when dropped."] # [derive (Debug)] pub struct AutoElementsCritical < 'array_local , T : TypeArray , TArrayRef > where TArrayRef : AsRef < JPrimitiveArray < 'array_local , T > > , { array : TArrayRef , len : usize , ptr : NonNull < T > , mode : ReleaseMode , is_copy : bool , _lifetime : PhantomData < & 'array_local () > , }
};
}
