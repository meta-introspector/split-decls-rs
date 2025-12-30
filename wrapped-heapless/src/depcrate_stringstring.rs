// Generated macro for String (type)
macro_rules! Depcrate_stringString {
() => {
// Module: crate::string
// Provides: {"String"}
// Dependencies: {}
# [doc = " A fixed capacity [`String`](https://doc.rust-lang.org/std/string/struct.String.html)."] pub type String < const N : usize , LenT = usize > = StringInner < LenT , OwnedStorage < N > > ;
};
}
