// Generated macro for ResultFromU32 (macro)
macro_rules! Depcrate_errorResultFromU32 {
() => {
// Module: crate::error
// Provides: {"ResultFromU32"}
// Dependencies: {}
# [doc = " Derive From<u32> for our result enum."] macro_rules ! ResultFromU32 { ($ (# [$ attr : meta]) * $ pub : vis enum $ NewType : ident { $ ($ Item : ident = $ Value : literal ,) * $ (,) ? }) => { impl From < u32 > for $ NewType { fn from (x : u32) -> Self { match x { $ ($ Value => Self ::$ Item ,) * _ => Self :: InvalidParameter , } } } } ; }
};
}
