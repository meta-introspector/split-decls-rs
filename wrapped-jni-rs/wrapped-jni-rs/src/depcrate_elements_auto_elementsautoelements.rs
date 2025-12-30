// Generated macro for AutoElements (struct)
macro_rules! Depcrate_elements_auto_elementsAutoElements {
() => {
// Module: crate::elements::auto_elements
// Provides: {"AutoElements"}
// Dependencies: {}
# [doc = " Auto-release wrapper for a mutable pointer to the elements of a [`JPrimitiveArray`]"] # [doc = " (such as [`JByteArray`])"] # [doc = ""] # [doc = " This type is used to wrap pointers returned by `Get<Type>ArrayElements`"] # [doc = " and ensure the pointer is released via `Release<Type>ArrayElements` when dropped."] # [doc = ""] # [doc = " The wrapper is tied to the lifetime of the array reference that becomes"] # [doc = " owned by the struct (the reference needs to be retained in order to call"] # [doc = " `Release<Type>ArrayElements` later)."] # [derive (Debug)] pub struct AutoElements < 'array_local , T , TArrayRef > where T : TypeArray + 'array_local , TArrayRef : AsRef < JPrimitiveArray < 'array_local , T > > , { array : TArrayRef , len : usize , ptr : NonNull < T > , mode : ReleaseMode , is_copy : bool , _lifetime : PhantomData < & 'array_local () > , }
};
}
