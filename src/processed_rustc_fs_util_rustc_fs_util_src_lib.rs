/* FP:lib.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_fs_util_src_lib_USE_0001
/* FP:lib.rs-0002 */ use std :: ffi :: { CString , OsStr } ;
/* FP:lib.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_fs_util_src_lib_USE_0002
/* FP:lib.rs-0004 */ use std :: path :: { Path , PathBuf , absolute } ;
/* FP:lib.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_fs_util_src_lib_USE_0003
/* FP:lib.rs-0006 */ use std :: { env , fs , io } ;
/* FP:lib.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_fs_util_src_lib_USE_0004
/* FP:lib.rs-0008 */ use tempfile :: TempDir ;
/* FP:lib.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_fs_util_src_lib_FN_0005
/* FP:lib.rs-0010 */ # [cfg (windows)] pub fn fix_windows_verbatim_for_gcc (p : & Path) -> PathBuf { use std :: ffi :: OsString ; use std :: path ; let mut components = p . components () ; let prefix = match components . next () { Some (path :: Component :: Prefix (p)) => p , _ => return p . to_path_buf () , } ; match prefix . kind () { path :: Prefix :: VerbatimDisk (disk) => { let mut base = OsString :: from (format ! ("{}:" , disk as char)) ; base . push (components . as_path ()) ; PathBuf :: from (base) } path :: Prefix :: VerbatimUNC (server , share) => { let mut base = OsString :: from (r"\\") ; base . push (server) ; base . push (r"\") ; base . push (share) ; base . push (components . as_path ()) ; PathBuf :: from (base) } _ => p . to_path_buf () , } }
/* FP:lib.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_fs_util_src_lib_FN_0006
/* FP:lib.rs-0012 */ # [cfg (not (windows))] pub fn fix_windows_verbatim_for_gcc (p : & Path) -> PathBuf { p . to_path_buf () }
/* FP:lib.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_fs_util_src_lib_ENUM_0007
/* FP:lib.rs-0014 */ pub enum LinkOrCopy { Link , Copy , }
/* FP:lib.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_fs_util_src_lib_FN_0008
/* FP:lib.rs-0016 */ # [doc = " Copies `p` into `q`, preferring to use hard-linking if possible."] # [doc = " The result indicates which of the two operations has been performed."] pub fn link_or_copy < P : AsRef < Path > , Q : AsRef < Path > > (p : P , q : Q) -> io :: Result < LinkOrCopy > { let p = p . as_ref () ; let q = q . as_ref () ; let err = match fs :: hard_link (p , q) { Ok (()) => return Ok (LinkOrCopy :: Link) , Err (err) => err , } ; if err . kind () == io :: ErrorKind :: AlreadyExists { fs :: remove_file (q) ? ; if fs :: hard_link (p , q) . is_ok () { return Ok (LinkOrCopy :: Link) ; } } fs :: copy (p , q) . map (| _ | LinkOrCopy :: Copy) }
/* FP:lib.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_fs_util_src_lib_FN_0009
/* FP:lib.rs-0018 */ # [cfg (any (unix , all (target_os = "wasi" , target_env = "p1")))] pub fn path_to_c_string (p : & Path) -> CString { use std :: ffi :: OsStr ; # [cfg (unix)] use std :: os :: unix :: ffi :: OsStrExt ; # [cfg (all (target_os = "wasi" , target_env = "p1"))] use std :: os :: wasi :: ffi :: OsStrExt ; let p : & OsStr = p . as_ref () ; CString :: new (p . as_bytes ()) . unwrap () }
/* FP:lib.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_fs_util_src_lib_FN_0010
/* FP:lib.rs-0020 */ # [cfg (windows)] pub fn path_to_c_string (p : & Path) -> CString { CString :: new (p . to_str () . unwrap ()) . unwrap () }
/* FP:lib.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_fs_util_src_lib_FN_0011
/* FP:lib.rs-0022 */ # [inline] pub fn try_canonicalize < P : AsRef < Path > > (path : P) -> io :: Result < PathBuf > { fs :: canonicalize (& path) . or_else (| _ | absolute (& path)) }
/* FP:lib.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_fs_util_src_lib_STRUCT_0012
/* FP:lib.rs-0024 */ pub struct TempDirBuilder < 'a , 'b > { builder : tempfile :: Builder < 'a , 'b > , }
/* FP:lib.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_fs_util_src_lib_IMPL_0013
/* FP:lib.rs-0026 */ impl < 'a , 'b > TempDirBuilder < 'a , 'b > { pub fn new () -> Self { Self { builder : tempfile :: Builder :: new () } } pub fn prefix < S : AsRef < OsStr > + ? Sized > (& mut self , prefix : & 'a S) -> & mut Self { self . builder . prefix (prefix) ; self } pub fn suffix < S : AsRef < OsStr > + ? Sized > (& mut self , suffix : & 'b S) -> & mut Self { self . builder . suffix (suffix) ; self } pub fn tempdir_in < P : AsRef < Path > > (& self , dir : P) -> io :: Result < TempDir > { let dir = dir . as_ref () ; # [cfg (windows)] for wait in 1 .. 11 { match self . builder . tempdir_in (dir) { Err (e) if e . kind () == io :: ErrorKind :: PermissionDenied => { } t => return t , } std :: thread :: sleep (std :: time :: Duration :: from_millis (1 << wait)) ; } self . builder . tempdir_in (dir) } pub fn tempdir (& self) -> io :: Result < TempDir > { self . tempdir_in (env :: temp_dir ()) } }