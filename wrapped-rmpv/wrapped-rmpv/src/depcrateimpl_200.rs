// Generated macro for impl_200 (impl)
macro_rules! Depcrateimpl_200 {
() => {
// Module: crate
// Provides: {"impl_200"}
// Dependencies: {}
impl Index < & str > for Value { type Output = Self ; fn index (& self , index : & str) -> & Self { if let Self :: Map (ref map) = * self { if let Some (found) = map . iter () . find (| (key , _val) | { if let Self :: String (ref strval) = * key { if let Some (s) = strval . as_str () { if s == index { return true ; } } } false }) { return & found . 1 ; } } & NIL } }
};
}
