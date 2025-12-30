// Generated macro for HeapStr (trait)
macro_rules! Depcrate_backendHeapStr {
() => {
// Module: crate::backend
// Provides: {"HeapStr"}
// Dependencies: {}
# [doc = " Abstract over different type of heap-allocated strings"] pub trait HeapStr : std :: fmt :: Debug + Clone + private :: Sealed { fn from_str (other : & str) -> Self ; fn from_string (other : String) -> Self ; fn from_boxed_str (other : BoxedStr) -> Self ; fn as_str (& self) -> & str ; }
};
}
