// Generated macro for strip_if_is_prefix (function)
macro_rules! Depcrate_dirstrip_if_is_prefix {
() => {
// Module: crate::dir
// Provides: {"strip_if_is_prefix"}
// Dependencies: {}
# [doc = " Strips `prefix` from `path` if it's a prefix, otherwise returns `path`"] # [doc = " unchanged."] fn strip_if_is_prefix < 'a , P : AsRef < Path > + ? Sized > (prefix : & 'a P , path : & 'a Path ,) -> & 'a Path { strip_prefix (prefix , path) . map_or (path , | p | p) }
};
}
