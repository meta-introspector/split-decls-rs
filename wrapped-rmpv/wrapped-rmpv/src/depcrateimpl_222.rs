// Generated macro for impl_222 (impl)
macro_rules! Depcrateimpl_222 {
() => {
// Module: crate
// Provides: {"impl_222"}
// Dependencies: {}
# [doc = " Note that an `Iterator<Item = u8>` will be collected into an"] # [doc = " [`Array`](crate::Value::Array), rather than a"] # [doc = " [`Binary`](crate::Value::Binary)"] impl < V > FromIterator < V > for Value where V : Into < Self > , { fn from_iter < I : IntoIterator < Item = V > > (iter : I) -> Self { let v : Vec < Value > = iter . into_iter () . map (| v | v . into ()) . collect () ; Self :: Array (v) } }
};
}
