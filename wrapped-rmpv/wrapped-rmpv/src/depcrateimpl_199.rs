// Generated macro for impl_199 (impl)
macro_rules! Depcrateimpl_199 {
() => {
// Module: crate
// Provides: {"impl_199"}
// Dependencies: {}
impl Index < usize > for Value { type Output = Self ; fn index (& self , index : usize) -> & Self { self . as_array () . and_then (| v | v . get (index)) . unwrap_or (& NIL) } }
};
}
