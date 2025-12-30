// Generated macro for strip_prefix_canonical (function)
macro_rules! Depcrate_pathsstrip_prefix_canonical {
() => {
// Module: crate::paths
// Provides: {"strip_prefix_canonical"}
// Dependencies: {}
# [doc = " Strips `base` from `path`."] # [doc = ""] # [doc = " This canonicalizes both paths before stripping. This is useful if the"] # [doc = " paths are obtained in different ways, and one or the other may or may not"] # [doc = " have been normalized in some way."] pub fn strip_prefix_canonical (path : impl AsRef < Path > , base : impl AsRef < Path > ,) -> Result < PathBuf , std :: path :: StripPrefixError > { let safe_canonicalize = | path : & Path | match path . canonicalize () { Ok (p) => p , Err (e) => { tracing :: warn ! ("cannot canonicalize {:?}: {:?}" , path , e) ; path . to_path_buf () } } ; let canon_path = safe_canonicalize (path . as_ref ()) ; let canon_base = safe_canonicalize (base . as_ref ()) ; canon_path . strip_prefix (canon_base) . map (| p | p . to_path_buf ()) }
};
}
