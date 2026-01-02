mkuse!{use std :: ffi :: { CString , OsStr } ;}
mkuse!{use std :: path :: { Path , PathBuf , absolute } ;}
mkuse!{use std :: { env , fs , io } ;}
mkuse!{use tempfile :: TempDir ;}

macro_rules! fix_windows_verbatim_for_gcc_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fix_windows_verbatim_for_gcc in module {}", module_path!());
    };
}

mkfn!{
    fix_windows_verbatim_for_gcc_introspect!();
    # [cfg (windows)] pub fn fix_windows_verbatim_for_gcc (p : & Path) -> PathBuf { use std :: ffi :: OsString ; use std :: path ; let mut components = p . components () ; let prefix = match components . next () { Some (path :: Component :: Prefix (p)) => p , _ => return p . to_path_buf () , } ; match prefix . kind () { path :: Prefix :: VerbatimDisk (disk) => { let mut base = OsString :: from (format ! ("{}:" , disk as char)) ; base . push (components . as_path ()) ; PathBuf :: from (base) } path :: Prefix :: VerbatimUNC (server , share) => { let mut base = OsString :: from (r"\\") ; base . push (server) ; base . push (r"\") ; base . push (share) ; base . push (components . as_path ()) ; PathBuf :: from (base) } _ => p . to_path_buf () , } }
}

macro_rules! fix_windows_verbatim_for_gcc_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fix_windows_verbatim_for_gcc in module {}", module_path!());
    };
}

mkfn!{
    fix_windows_verbatim_for_gcc_introspect!();
    # [cfg (not (windows))] pub fn fix_windows_verbatim_for_gcc (p : & Path) -> PathBuf { p . to_path_buf () }
}
mkitem!{mkenum!{pub enum LinkOrCopy { Link , Copy , }}}

macro_rules! link_or_copy_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function link_or_copy in module {}", module_path!());
    };
}

mkfn!{
    link_or_copy_introspect!();
    # [doc = " Copies `p` into `q`, preferring to use hard-linking if possible."] # [doc = " The result indicates which of the two operations has been performed."] pub fn link_or_copy < P : AsRef < Path > , Q : AsRef < Path > > (p : P , q : Q) -> io :: Result < LinkOrCopy > { let p = p . as_ref () ; let q = q . as_ref () ; let err = match fs :: hard_link (p , q) { Ok (()) => return Ok (LinkOrCopy :: Link) , Err (err) => err , } ; if err . kind () == io :: ErrorKind :: AlreadyExists { fs :: remove_file (q) ? ; if fs :: hard_link (p , q) . is_ok () { return Ok (LinkOrCopy :: Link) ; } } fs :: copy (p , q) . map (| _ | LinkOrCopy :: Copy) }
}

macro_rules! path_to_c_string_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function path_to_c_string in module {}", module_path!());
    };
}

mkfn!{
    path_to_c_string_introspect!();
    # [cfg (any (unix , all (target_os = "wasi" , target_env = "p1")))] pub fn path_to_c_string (p : & Path) -> CString { use std :: ffi :: OsStr ; # [cfg (unix)] use std :: os :: unix :: ffi :: OsStrExt ; # [cfg (all (target_os = "wasi" , target_env = "p1"))] use std :: os :: wasi :: ffi :: OsStrExt ; let p : & OsStr = p . as_ref () ; CString :: new (p . as_bytes ()) . unwrap () }
}

macro_rules! path_to_c_string_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function path_to_c_string in module {}", module_path!());
    };
}

mkfn!{
    path_to_c_string_introspect!();
    # [cfg (windows)] pub fn path_to_c_string (p : & Path) -> CString { CString :: new (p . to_str () . unwrap ()) . unwrap () }
}

macro_rules! try_canonicalize_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function try_canonicalize in module {}", module_path!());
    };
}

mkfn!{
    try_canonicalize_introspect!();
    # [inline] pub fn try_canonicalize < P : AsRef < Path > > (path : P) -> io :: Result < PathBuf > { fs :: canonicalize (& path) . or_else (| _ | absolute (& path)) }
}
mkitem!{mkstruct!{pub struct TempDirBuilder < 'a , 'b > { builder : tempfile :: Builder < 'a , 'b > , }}}
mkitem!{mkimpl!{impl < 'a , 'b > TempDirBuilder < 'a , 'b > { pub fn new () -> Self { Self { builder : tempfile :: Builder :: new () } } pub fn prefix < S : AsRef < OsStr > + ? Sized > (& mut self , prefix : & 'a S) -> & mut Self { self . builder . prefix (prefix) ; self } pub fn suffix < S : AsRef < OsStr > + ? Sized > (& mut self , suffix : & 'b S) -> & mut Self { self . builder . suffix (suffix) ; self } pub fn tempdir_in < P : AsRef < Path > > (& self , dir : P) -> io :: Result < TempDir > { let dir = dir . as_ref () ; # [cfg (windows)] for wait in 1 .. 11 { match self . builder . tempdir_in (dir) { Err (e) if e . kind () == io :: ErrorKind :: PermissionDenied => { } t => return t , } std :: thread :: sleep (std :: time :: Duration :: from_millis (1 << wait)) ; } self . builder . tempdir_in (dir) } pub fn tempdir (& self) -> io :: Result < TempDir > { self . tempdir_in (env :: temp_dir ()) } }}}