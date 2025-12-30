// Generated macro for NoExpand (struct)
macro_rules! Depcrate_re_bytesNoExpand {
() => {
// Module: crate::re_bytes
// Provides: {"NoExpand"}
// Dependencies: {}
# [doc = " NoExpand indicates literal byte string replacement."] # [doc = ""] # [doc = " It can be used with `replace` and `replace_all` to do a literal byte string"] # [doc = " replacement without expanding `$name` to their corresponding capture"] # [doc = " groups. This can be both convenient (to avoid escaping `$`, for example)"] # [doc = " and performant (since capture groups don't need to be found)."] # [doc = ""] # [doc = " `'t` is the lifetime of the literal text."] pub struct NoExpand < 'r > (pub & 'r [u8]) ;
};
}
