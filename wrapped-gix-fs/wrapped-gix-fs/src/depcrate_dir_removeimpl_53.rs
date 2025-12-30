// Generated macro for impl_53 (impl)
macro_rules! Depcrate_dir_removeimpl_53 {
() => {
// Module: crate::dir::remove
// Provides: {"impl_53"}
// Dependencies: {}
# [doc = " Construction"] impl < 'a > Iter < 'a > { # [doc = " Create a new instance that deletes `target` but will stop at `boundary`, without deleting the latter."] # [doc = " Returns an error if `boundary` doesn't contain `target`"] # [doc = ""] # [doc = " **Note** that we don't canonicalize the path for performance reasons."] pub fn new (target : & 'a Path , boundary : & 'a Path) -> std :: io :: Result < Self > { if ! target . starts_with (boundary) { return Err (std :: io :: Error :: new (std :: io :: ErrorKind :: InvalidInput , format ! ("Removal target '{target}' must be contained in boundary '{boundary}'" , target = target . display () , boundary = boundary . display ()) ,)) ; } let cursor = if target == boundary { None } else if target . exists () { Some (target) } else { None } ; Ok (Iter { cursor , boundary }) } }
};
}
