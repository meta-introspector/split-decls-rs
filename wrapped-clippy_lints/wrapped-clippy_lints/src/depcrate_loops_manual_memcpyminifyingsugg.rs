// Generated macro for MinifyingSugg (struct)
macro_rules! Depcrate_loops_manual_memcpyMinifyingSugg {
() => {
// Module: crate::loops::manual_memcpy
// Provides: {"MinifyingSugg"}
// Dependencies: {}
# [doc = " a wrapper of `Sugg`. Besides what `Sugg` do, this removes unnecessary `0`;"] # [doc = " and also, it avoids subtracting a variable from the same one by replacing it with `0`."] # [doc = " it exists for the convenience of the overloaded operators while normal functions can do the"] # [doc = " same."] # [derive (Clone)] struct MinifyingSugg < 'a > (Sugg < 'a >) ;
};
}
