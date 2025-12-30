// Generated macro for Interner (struct)
macro_rules! Depcrate_internInterner {
() => {
// Module: crate::intern
// Provides: {"Interner"}
// Dependencies: {}
# [doc = " An interner that allows for fast access of tokens produced by a [`TokenSource`]."] # [derive (Default)] pub struct Interner < T > { tokens : Vec < T > , table : HashTable < Token > , hasher : RandomState , }
};
}
