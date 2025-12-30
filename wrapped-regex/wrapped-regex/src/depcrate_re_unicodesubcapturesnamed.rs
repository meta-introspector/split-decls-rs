// Generated macro for SubCapturesNamed (struct)
macro_rules! Depcrate_re_unicodeSubCapturesNamed {
() => {
// Module: crate::re_unicode
// Provides: {"SubCapturesNamed"}
// Dependencies: {}
# [doc = " An Iterator over named capture groups as a tuple with the group"] # [doc = " name and the value."] # [doc = ""] # [doc = " `'c` is the lifetime of the captures."] pub struct SubCapturesNamed < 'c > { caps : & 'c Captures < 'c > , names : NamedGroupsIter < 'c > , }
};
}
