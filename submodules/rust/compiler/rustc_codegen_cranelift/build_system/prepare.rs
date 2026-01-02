mkuse!{use std :: ffi :: OsStr ;}
mkuse!{use std :: hash :: { Hash , Hasher } ;}
mkuse!{use std :: path :: { Path , PathBuf } ;}
mkuse!{use std :: process :: Command ;}
mkuse!{use std :: { fs , io } ;}
mkuse!{use crate :: path :: { Dirs , RelPath } ;}
mkuse!{use crate :: utils :: { copy_dir_recursively , ensure_empty_dir , spawn_and_wait } ;}

macro_rules! prepare_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function prepare in module {}", module_path!());
    };
}

mkfn!{
    prepare_introspect!();
    pub (crate) fn prepare (dirs : & Dirs) { std :: fs :: create_dir_all (& dirs . download_dir) . unwrap () ; crate :: tests :: RAND_REPO . fetch (dirs) ; crate :: tests :: REGEX_REPO . fetch (dirs) ; }
}
mkitem!{mkstruct!{pub (crate) struct GitRepo { url : GitRepoUrl , rev : & 'static str , content_hash : & 'static str , patch_name : & 'static str , }}}
mkitem!{mkenum!{enum GitRepoUrl { Github { user : & 'static str , repo : & 'static str } , }}}

macro_rules! hash_file_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function hash_file in module {}", module_path!());
    };
}

mkfn!{
    hash_file_introspect!();
    fn hash_file (file : & std :: path :: Path) -> u64 { let contents = std :: fs :: read (file) . unwrap () ; # [allow (deprecated)] let mut hasher = std :: hash :: SipHasher :: new () ; hasher . write_usize (contents . len () . to_le ()) ; Hash :: hash_slice (& contents , & mut hasher) ; std :: hash :: Hasher :: finish (& hasher) }
}

macro_rules! hash_dir_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function hash_dir in module {}", module_path!());
    };
}

mkfn!{
    hash_dir_introspect!();
    fn hash_dir (dir : & std :: path :: Path) -> u64 { let mut sub_hashes = std :: collections :: BTreeMap :: new () ; for entry in std :: fs :: read_dir (dir) . unwrap () { let entry = entry . unwrap () ; if entry . file_type () . unwrap () . is_dir () { sub_hashes . insert (entry . file_name () . to_str () . unwrap () . to_owned () , hash_dir (& entry . path ()) . to_le () ,) ; } else { sub_hashes . insert (entry . file_name () . to_str () . unwrap () . to_owned () , hash_file (& entry . path ()) . to_le () ,) ; } } # [allow (deprecated)] let mut hasher = std :: hash :: SipHasher :: new () ; hasher . write_usize (sub_hashes . len () . to_le ()) ; for elt in sub_hashes { elt . hash (& mut hasher) ; } std :: hash :: Hasher :: finish (& hasher) }
}
mkitem!{mkimpl!{impl GitRepo { pub (crate) const fn github (user : & 'static str , repo : & 'static str , rev : & 'static str , content_hash : & 'static str , patch_name : & 'static str ,) -> GitRepo { GitRepo { url : GitRepoUrl :: Github { user , repo } , rev , content_hash , patch_name } } fn download_dir (& self , dirs : & Dirs) -> PathBuf { match self . url { GitRepoUrl :: Github { user : _ , repo } => dirs . download_dir . join (repo) , } } pub (crate) const fn source_dir (& self) -> RelPath { match self . url { GitRepoUrl :: Github { user : _ , repo } => RelPath :: build (repo) , } } fn verify_checksum (& self , dirs : & Dirs) { let download_dir = self . download_dir (dirs) ; if ! download_dir . exists () { eprintln ! ("Missing directory {download_dir}: Please run ./y.sh prepare to download." , download_dir = download_dir . display () ,) ; std :: process :: exit (1) ; } let actual_hash = format ! ("{:016x}" , hash_dir (& download_dir)) ; if actual_hash != self . content_hash { eprintln ! ("Mismatched content hash for {download_dir}: {actual_hash} != {content_hash}. Please run ./y.sh prepare again." , download_dir = download_dir . display () , content_hash = self . content_hash ,) ; std :: process :: exit (1) ; } } pub (crate) fn fetch (& self , dirs : & Dirs) { let download_dir = self . download_dir (dirs) ; if download_dir . exists () { let actual_hash = format ! ("{:016x}" , hash_dir (& download_dir)) ; if actual_hash == self . content_hash { eprintln ! ("[FRESH] {}" , download_dir . display ()) ; return ; } else { eprintln ! ("Mismatched content hash for {download_dir}: {actual_hash} != {content_hash}. Downloading again." , download_dir = download_dir . display () , content_hash = self . content_hash ,) ; } } match self . url { GitRepoUrl :: Github { user , repo } => { clone_repo (& download_dir , & format ! ("https://github.com/{}/{}.git" , user , repo) , self . rev ,) ; } } let source_lockfile = dirs . source_dir . join ("patches") . join (format ! ("{}-lock.toml" , self . patch_name)) ; let target_lockfile = download_dir . join ("Cargo.lock") ; if source_lockfile . exists () { assert ! (! target_lockfile . exists ()) ; fs :: copy (source_lockfile , target_lockfile) . unwrap () ; } else { assert ! (target_lockfile . exists ()) ; } self . verify_checksum (dirs) ; } pub (crate) fn patch (& self , dirs : & Dirs) { self . verify_checksum (dirs) ; apply_patches (dirs , self . patch_name , & self . download_dir (dirs) , & self . source_dir () . to_path (dirs) ,) ; } }}}

macro_rules! clone_repo_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function clone_repo in module {}", module_path!());
    };
}

mkfn!{
    clone_repo_introspect!();
    fn clone_repo (download_dir : & Path , repo : & str , rev : & str) { eprintln ! ("[CLONE] {}" , repo) ; match fs :: remove_dir_all (download_dir) { Ok (()) => { } Err (err) if err . kind () == io :: ErrorKind :: NotFound => { } Err (err) => panic ! ("Failed to remove {path}: {err}" , path = download_dir . display ()) , } git_command (None , "clone") . arg (repo) . arg (download_dir) . spawn () . unwrap () . wait () . unwrap () ; let mut clean_cmd = git_command (download_dir , "checkout") ; clean_cmd . arg ("--") . arg (".") ; spawn_and_wait (clean_cmd) ; let mut checkout_cmd = git_command (download_dir , "checkout") ; checkout_cmd . arg ("-q") . arg (rev) ; spawn_and_wait (checkout_cmd) ; std :: fs :: remove_dir_all (download_dir . join (".git")) . unwrap () ; }
}

macro_rules! init_git_repo_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function init_git_repo in module {}", module_path!());
    };
}

mkfn!{
    init_git_repo_introspect!();
    fn init_git_repo (repo_dir : & Path) { let mut git_init_cmd = git_command (repo_dir , "init") ; git_init_cmd . arg ("-q") ; spawn_and_wait (git_init_cmd) ; let mut git_add_cmd = git_command (repo_dir , "add") ; git_add_cmd . arg (".") ; spawn_and_wait (git_add_cmd) ; let mut git_commit_cmd = git_command (repo_dir , "commit") ; git_commit_cmd . arg ("-m") . arg ("Initial commit") . arg ("-q") . arg ("--no-verify") ; spawn_and_wait (git_commit_cmd) ; }
}

macro_rules! get_patches_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function get_patches in module {}", module_path!());
    };
}

mkfn!{
    get_patches_introspect!();
    fn get_patches (dirs : & Dirs , crate_name : & str) -> Vec < PathBuf > { let mut patches : Vec < _ > = fs :: read_dir (dirs . source_dir . join ("patches")) . unwrap () . map (| entry | entry . unwrap () . path ()) . filter (| path | path . extension () == Some (OsStr :: new ("patch"))) . filter (| path | { path . file_name () . unwrap () . to_str () . unwrap () . split_once ("-") . unwrap () . 1 . starts_with (crate_name) }) . collect () ; patches . sort () ; patches }
}

macro_rules! apply_patches_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function apply_patches in module {}", module_path!());
    };
}

mkfn!{
    apply_patches_introspect!();
    pub (crate) fn apply_patches (dirs : & Dirs , crate_name : & str , source_dir : & Path , target_dir : & Path) { eprintln ! ("[COPY] {crate_name} source") ; ensure_empty_dir (target_dir) ; if crate_name == "stdlib" { fs :: create_dir (target_dir . join ("library")) . unwrap () ; copy_dir_recursively (& source_dir . join ("library") , & target_dir . join ("library")) ; } else { copy_dir_recursively (source_dir , target_dir) ; } init_git_repo (target_dir) ; if crate_name == "<none>" { return ; } for patch in get_patches (dirs , crate_name) { eprintln ! ("[PATCH] {:?} <- {:?}" , target_dir . file_name () . unwrap () , patch . file_name () . unwrap ()) ; let mut apply_patch_cmd = git_command (target_dir , "am") ; apply_patch_cmd . arg (patch) . arg ("-q") ; spawn_and_wait (apply_patch_cmd) ; } }
}

macro_rules! git_command_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function git_command in module {}", module_path!());
    };
}

mkfn!{
    git_command_introspect!();
    # [must_use] fn git_command < 'a > (repo_dir : impl Into < Option < & 'a Path > > , cmd : & str) -> Command { let mut git_cmd = Command :: new ("git") ; git_cmd . arg ("-c") . arg ("user.name=Dummy") . arg ("-c") . arg ("user.email=dummy@example.com") . arg ("-c") . arg ("core.autocrlf=false") . arg ("-c") . arg ("commit.gpgSign=false") . arg (cmd) ; if let Some (repo_dir) = repo_dir . into () { git_cmd . current_dir (repo_dir) ; } git_cmd }
}