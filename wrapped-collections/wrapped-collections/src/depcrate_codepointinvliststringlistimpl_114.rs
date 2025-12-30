// Generated macro for impl_114 (impl)
macro_rules! Depcrate_codepointinvliststringlistimpl_114 {
() => {
// Module: crate::codepointinvliststringlist
// Provides: {"impl_114"}
// Dependencies: {}
# [cfg (feature = "alloc")] # [doc = " ✨ *Enabled with the `alloc` Cargo feature.*"] impl < 'a > FromIterator < & 'a str > for CodePointInversionListAndStringList < '_ > { fn from_iter < I > (it : I) -> Self where I : IntoIterator < Item = & 'a str > , { let mut builder = CodePointInversionListBuilder :: new () ; let mut strings = Vec :: < & str > :: new () ; for s in it { let mut chars = s . chars () ; if let Some (first_char) = chars . next () { if chars . next () . is_none () { builder . add_char (first_char) ; continue ; } } strings . push (s) ; } strings . sort_unstable () ; strings . dedup () ; let cp_inv_list = builder . build () ; let str_list = VarZeroVec :: < str > :: from (& strings) ; CodePointInversionListAndStringList { cp_inv_list , str_list , } } }
};
}
