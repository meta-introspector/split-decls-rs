mkuse!{use crate :: io ;}
mkuse!{use crate :: path :: { Path , PathBuf } ;}
mkmod!{common, { 
                getname!(common);
                getsrc!(common);
                getpath!(common);
                get_deps!(common);
                get_crates!(common);
                mkinclude!(common);
                 
            }}
mkitem!{cfg_select ! { target_family = "unix" => { mod unix ; use unix as imp ; pub use unix :: { chown , fchown , lchown , mkfifo } ; # [cfg (not (target_os = "fuchsia"))] pub use unix :: chroot ; pub (crate) use unix :: debug_assert_fd_is_open ; # [cfg (any (target_os = "linux" , target_os = "android"))] pub (crate) use unix :: CachedFileMetadata ; use crate :: sys :: common :: small_c_string :: run_path_with_cstr as with_native_path ; } target_os = "windows" => { mod windows ; use windows as imp ; pub use windows :: { symlink_inner , junction_point } ; use crate :: sys :: path :: with_native_path ; } target_os = "hermit" => { mod hermit ; use hermit as imp ; } target_os = "solid_asp3" => { mod solid ; use solid as imp ; } target_os = "uefi" => { mod uefi ; use uefi as imp ; } target_os = "wasi" => { mod wasi ; use wasi as imp ; } _ => { mod unsupported ; use unsupported as imp ; } }}

macro_rules! with_native_path_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function with_native_path in module {}", module_path!());
    };
}

mkfn!{
    with_native_path_introspect!();
    # [cfg (not (any (target_family = "unix" , target_os = "windows")))] # [inline] pub fn with_native_path < T > (path : & Path , f : & dyn Fn (& Path) -> io :: Result < T >) -> io :: Result < T > { f (path) }
}
mkuse!{pub use imp :: { DirBuilder , DirEntry , File , FileAttr , FilePermissions , FileTimes , FileType , OpenOptions , ReadDir , } ;}

macro_rules! read_dir_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function read_dir in module {}", module_path!());
    };
}

mkfn!{
    read_dir_introspect!();
    pub fn read_dir (path : & Path) -> io :: Result < ReadDir > { imp :: readdir (path) }
}

macro_rules! remove_file_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function remove_file in module {}", module_path!());
    };
}

mkfn!{
    remove_file_introspect!();
    pub fn remove_file (path : & Path) -> io :: Result < () > { with_native_path (path , & imp :: unlink) }
}

macro_rules! rename_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function rename in module {}", module_path!());
    };
}

mkfn!{
    rename_introspect!();
    pub fn rename (old : & Path , new : & Path) -> io :: Result < () > { with_native_path (old , & | old | with_native_path (new , & | new | imp :: rename (old , new))) }
}

macro_rules! remove_dir_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function remove_dir in module {}", module_path!());
    };
}

mkfn!{
    remove_dir_introspect!();
    pub fn remove_dir (path : & Path) -> io :: Result < () > { with_native_path (path , & imp :: rmdir) }
}

macro_rules! remove_dir_all_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function remove_dir_all in module {}", module_path!());
    };
}

mkfn!{
    remove_dir_all_introspect!();
    pub fn remove_dir_all (path : & Path) -> io :: Result < () > { # [cfg (not (windows))] return imp :: remove_dir_all (path) ; # [cfg (windows)] with_native_path (path , & imp :: remove_dir_all) }
}

macro_rules! read_link_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function read_link in module {}", module_path!());
    };
}

mkfn!{
    read_link_introspect!();
    pub fn read_link (path : & Path) -> io :: Result < PathBuf > { with_native_path (path , & imp :: readlink) }
}

macro_rules! symlink_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function symlink in module {}", module_path!());
    };
}

mkfn!{
    symlink_introspect!();
    pub fn symlink (original : & Path , link : & Path) -> io :: Result < () > { # [cfg (windows)] return imp :: symlink (original , link) ; # [cfg (not (windows))] with_native_path (original , & | original | { with_native_path (link , & | link | imp :: symlink (original , link)) }) }
}

macro_rules! hard_link_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function hard_link in module {}", module_path!());
    };
}

mkfn!{
    hard_link_introspect!();
    pub fn hard_link (original : & Path , link : & Path) -> io :: Result < () > { with_native_path (original , & | original | { with_native_path (link , & | link | imp :: link (original , link)) }) }
}

macro_rules! metadata_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function metadata in module {}", module_path!());
    };
}

mkfn!{
    metadata_introspect!();
    pub fn metadata (path : & Path) -> io :: Result < FileAttr > { with_native_path (path , & imp :: stat) }
}

macro_rules! symlink_metadata_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function symlink_metadata in module {}", module_path!());
    };
}

mkfn!{
    symlink_metadata_introspect!();
    pub fn symlink_metadata (path : & Path) -> io :: Result < FileAttr > { with_native_path (path , & imp :: lstat) }
}

macro_rules! set_permissions_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function set_permissions in module {}", module_path!());
    };
}

mkfn!{
    set_permissions_introspect!();
    pub fn set_permissions (path : & Path , perm : FilePermissions) -> io :: Result < () > { with_native_path (path , & | path | imp :: set_perm (path , perm . clone ())) }
}

macro_rules! set_permissions_nofollow_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function set_permissions_nofollow in module {}", module_path!());
    };
}

mkfn!{
    set_permissions_nofollow_introspect!();
    # [cfg (unix)] pub fn set_permissions_nofollow (path : & Path , perm : crate :: fs :: Permissions) -> io :: Result < () > { use crate :: fs :: OpenOptions ; let mut options = OpenOptions :: new () ; # [cfg (not (any (target_os = "espidf" , target_os = "horizon")))] { use crate :: os :: unix :: fs :: OpenOptionsExt ; options . custom_flags (libc :: O_NOFOLLOW) ; } options . open (path) ? . set_permissions (perm) }
}

macro_rules! set_permissions_nofollow_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function set_permissions_nofollow in module {}", module_path!());
    };
}

mkfn!{
    set_permissions_nofollow_introspect!();
    # [cfg (not (unix))] pub fn set_permissions_nofollow (_path : & Path , _perm : crate :: fs :: Permissions) -> io :: Result < () > { crate :: unimplemented ! ("`set_permissions_nofollow` is currently only implemented on Unix platforms") }
}

macro_rules! canonicalize_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function canonicalize in module {}", module_path!());
    };
}

mkfn!{
    canonicalize_introspect!();
    pub fn canonicalize (path : & Path) -> io :: Result < PathBuf > { with_native_path (path , & imp :: canonicalize) }
}

macro_rules! copy_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function copy in module {}", module_path!());
    };
}

mkfn!{
    copy_introspect!();
    pub fn copy (from : & Path , to : & Path) -> io :: Result < u64 > { # [cfg (not (windows))] return imp :: copy (from , to) ; # [cfg (windows)] with_native_path (from , & | from | with_native_path (to , & | to | imp :: copy (from , to))) }
}

macro_rules! exists_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function exists in module {}", module_path!());
    };
}

mkfn!{
    exists_introspect!();
    pub fn exists (path : & Path) -> io :: Result < bool > { # [cfg (not (windows))] return imp :: exists (path) ; # [cfg (windows)] with_native_path (path , & imp :: exists) }
}