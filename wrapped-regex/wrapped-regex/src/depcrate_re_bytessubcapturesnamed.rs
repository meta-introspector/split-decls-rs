// Generated macro for SubCapturesNamed (struct)
macro_rules! Depcrate_re_bytesSubCapturesNamed {
() => {
// Module: crate::re_bytes
// Provides: {"SubCapturesNamed"}
// Dependencies: {}
# [doc = " An Iterator over named capture groups as a tuple with the group name and"] # [doc = " the value."] # [doc = ""] # [doc = " `'c` is the lifetime of the captures and `'t` is the lifetime of the"] # [doc = " matched text."] pub struct SubCapturesNamed < 'c , 't : 'c > { caps : & 'c Captures < 't > , names : hash_map :: Iter < 'c , String , usize > , }
};
}
