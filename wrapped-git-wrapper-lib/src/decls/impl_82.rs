macro_rules! deps {
    () => {
        Result!();
        SystemGhExecutor!();
        GhExecutor!();
    };
}

macro_rules! impl_82 {
    () => {
        deps!();
        impl GhExecutor for SystemGhExecutor { fn repo_fork (& self , repo_url : & str , target_org : & str) -> Result < () > { println ! ("Executing gh repo fork {} --org ?{}" , repo_url , target_org) ; let program = self . gh_executable_path . as_os_str () ; let args = & [OsStr :: new ("repo") , OsStr :: new ("fork") , OsStr :: new (repo_url) , OsStr :: new ("--org") , OsStr :: new (target_org) , OsStr :: new ("--remote") , OsStr :: new ("--clone=false") ,] ; let fork_output = self . executor . execv (program , args , None) ? ; if ! fork_output . status . success () { eprintln ! ("Failed to fork {}: {}" , repo_url , String :: from_utf8_lossy (& fork_output . stderr)) ; # [cfg (feature = "anyhow_enabled")] anyhow :: bail ! ("Forking failed for {}" , repo_url) ; # [cfg (not (feature = "anyhow_enabled"))] return Err (Box :: new (std :: io :: Error :: new (std :: io :: ErrorKind :: Other , format ! ("Forking failed for {}" , repo_url) ,))) ; } println ! ("Successfully forked {}." , repo_url) ; Ok (()) } fn repo_view (& self , forked_repo_url : & str) -> Result < bool > { println ! ("Executing gh repo view {} --json name" , forked_repo_url) ; let program = self . gh_executable_path . as_os_str () ; let args = & [OsStr :: new ("repo") , OsStr :: new ("view") , OsStr :: new (forked_repo_url) , OsStr :: new ("--json") , OsStr :: new ("name") ,] ; let gh_repo_check_output = self . executor . execv (program , args , None) ? ; Ok (gh_repo_check_output . status . success () && ! String :: from_utf8_lossy (& gh_repo_check_output . stdout) . trim () . is_empty ()) } }
    };
}

impl_82!();