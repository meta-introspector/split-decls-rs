macro_rules! deps {
    () => {
        Result!();
        GitAdapter!();
        SubmoduleStat!();
        ShellGitAdapter!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl GitAdapter for ShellGitAdapter { fn list_submodules (& self , root_dir : & Path) -> Result < Vec < (String , PathBuf) > > { println ! ("[ShellGitAdapter] Listing submodules in {:?}" , root_dir) ; let output = self . execv . execv (OsStr :: new ("git") , & [OsStr :: new ("submodule") , OsStr :: new ("status") , OsStr :: new ("--recursive") ,] , Some (root_dir) ,) ? ; let stdout = String :: from_utf8_lossy (& output . stdout) ; let mut submodules = Vec :: new () ; for line in stdout . lines () { let parts : Vec < & str > = line . trim () . split_whitespace () . collect () ; if parts . len () >= 2 { let path_str = parts [1] ; let path = PathBuf :: from (path_str) ; submodules . push ((path_str . to_string () , path)) ; } } Ok (submodules) } fn get_submodule_head_and_workdir_hash (& self , path : & Path) -> Result < SubmoduleStat > { println ! ("[ShellGitAdapter] Getting submodule stat for {:?}" , path) ; let head_output = self . execv . execv (OsStr :: new ("git") , & [OsStr :: new ("rev-parse") , OsStr :: new ("HEAD")] , Some (path) ,) ? ; let head_commit = String :: from_utf8_lossy (& head_output . stdout) . trim () . to_string () ; let status_output = self . execv . execv (OsStr :: new ("git") , & [OsStr :: new ("status") , OsStr :: new ("--porcelain")] , Some (path) ,) ? ; let dirty = ! status_output . stdout . is_empty () ; let workdir_hash = if dirty { format ! ("{}-dirty" , head_commit) } else { head_commit . clone () } ; Ok (SubmoduleStat { head_commit , workdir_hash , }) } fn as_any (& self) -> & dyn Any { self } }
    };
}

impl_22!()