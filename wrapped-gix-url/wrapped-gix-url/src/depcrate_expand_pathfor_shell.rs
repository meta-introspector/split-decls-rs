// Generated macro for for_shell (function)
macro_rules! Depcrate_expand_pathfor_shell {
() => {
// Module: crate::expand_path
// Provides: {"for_shell"}
// Dependencies: {}
# [doc = " Expand `path` for use in a shell and return the expanded path."] pub fn for_shell (path : BString) -> BString { use bstr :: ByteVec ; match parse (path . as_slice () . as_bstr ()) { Ok ((user , mut path)) => match user { Some (ForUser :: Current) => { path . insert (0 , b'~') ; path } Some (ForUser :: Name (mut user)) => { user . insert (0 , b'~') ; user . append (path . as_vec_mut ()) ; user } None => path , } , Err (_) => path , } }
};
}
