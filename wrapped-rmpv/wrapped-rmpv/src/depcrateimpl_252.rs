// Generated macro for impl_252 (impl)
macro_rules! Depcrateimpl_252 {
() => {
// Module: crate
// Provides: {"impl_252"}
// Dependencies: {}
# [doc = " Note that an `Iterator<Item = u8>` will be collected into an"] # [doc = " [`Array`](crate::Value::Array), rather than a"] # [doc = " [`Binary`](crate::Value::Binary)"] impl < 'a , V > FromIterator < V > for ValueRef < 'a > where V : Into < Self > , { fn from_iter < I : IntoIterator < Item = V > > (iter : I) -> Self { let v : Vec < ValueRef < 'a > > = iter . into_iter () . map (| v | v . into ()) . collect () ; ValueRef :: Array (v) } }
};
}
