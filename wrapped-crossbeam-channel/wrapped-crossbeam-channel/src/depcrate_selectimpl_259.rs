// Generated macro for impl_259 (impl)
macro_rules! Depcrate_selectimpl_259 {
() => {
// Module: crate::select
// Provides: {"impl_259"}
// Dependencies: {}
impl Operation { # [doc = " Creates an operation identifier from a mutable reference."] # [doc = ""] # [doc = " This function essentially just turns the address of the reference into a number. The"] # [doc = " reference should point to a variable that is specific to the thread and the operation,"] # [doc = " and is alive for the entire duration of select or blocking operation."] # [inline] pub fn hook < T > (r : & mut T) -> Self { let val = r as * mut T as usize ; assert ! (val > 2) ; Self (val) } }
};
}
