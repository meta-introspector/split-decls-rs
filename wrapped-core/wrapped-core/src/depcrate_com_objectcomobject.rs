// Generated macro for ComObject (struct)
macro_rules! Depcrate_com_objectComObject {
() => {
// Module: crate::com_object
// Provides: {"ComObject"}
// Dependencies: {}
# [doc = " A counted pointer to a type that implements COM interfaces, where the object has been"] # [doc = " placed in the heap (boxed)."] # [doc = ""] # [doc = " This type exists so that you can place an object into the heap and query for COM interfaces,"] # [doc = " without losing the safe reference to the implementation object."] # [doc = ""] # [doc = " Because the pointer inside this type is known to be non-null, `Option<ComObject<T>>` should"] # [doc = " always have the same size as a single pointer."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The contained `ptr` field is an owned, reference-counted pointer to a _pinned_ `Pin<Box<T::Outer>>`."] # [doc = " Although this code does not currently use `Pin<T>`, it takes care not to expose any unsafe semantics"] # [doc = " to safe code. However, code that calls unsafe functions on [`ComObject`] must, like all unsafe code,"] # [doc = " understand and preserve invariants."] # [repr (transparent)] pub struct ComObject < T : ComObjectInner > { ptr : NonNull < T :: Outer > , }
};
}
