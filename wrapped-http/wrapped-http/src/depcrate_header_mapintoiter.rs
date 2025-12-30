// Generated macro for IntoIter (struct)
macro_rules! Depcrate_header_mapIntoIter {
() => {
// Module: crate::header::map
// Provides: {"IntoIter"}
// Dependencies: {}
# [doc = " An owning iterator over the entries of a `HeaderMap`."] # [doc = ""] # [doc = " This struct is created by the `into_iter` method on `HeaderMap`."] # [derive (Debug)] pub struct IntoIter < T > { next : Option < usize > , entries : vec :: IntoIter < Bucket < T > > , extra_values : Vec < ExtraValue < T > > , }
};
}
