// Generated macro for tests (module)
macro_rules! Depcrate_pathutiltests {
() => {
// Module: crate::pathutil
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use std :: borrow :: Cow ; use bstr :: { B , ByteVec } ; use super :: { file_name_ext , normalize_path } ; macro_rules ! ext { ($ name : ident , $ file_name : expr , $ ext : expr) => { # [test] fn $ name () { let bs = Vec :: from ($ file_name) ; let got = file_name_ext (& Cow :: Owned (bs)) ; assert_eq ! ($ ext . map (| s | Cow :: Borrowed (B (s))) , got) ; } } ; } ext ! (ext1 , "foo.rs" , Some (".rs")) ; ext ! (ext2 , ".rs" , Some (".rs")) ; ext ! (ext3 , "..rs" , Some (".rs")) ; ext ! (ext4 , "" , None ::<& str >) ; ext ! (ext5 , "foo" , None ::<& str >) ; macro_rules ! normalize { ($ name : ident , $ path : expr , $ expected : expr) => { # [test] fn $ name () { let bs = Vec :: from_slice ($ path) ; let got = normalize_path (Cow :: Owned (bs)) ; assert_eq ! ($ expected . to_vec () , got . into_owned ()) ; } } ; } normalize ! (normal1 , b"foo" , b"foo") ; normalize ! (normal2 , b"foo/bar" , b"foo/bar") ; # [cfg (unix)] normalize ! (normal3 , b"foo\\bar" , b"foo\\bar") ; # [cfg (not (unix))] normalize ! (normal3 , b"foo\\bar" , b"foo/bar") ; # [cfg (unix)] normalize ! (normal4 , b"foo\\bar/baz" , b"foo\\bar/baz") ; # [cfg (not (unix))] normalize ! (normal4 , b"foo\\bar/baz" , b"foo/bar/baz") ; }
};
}
