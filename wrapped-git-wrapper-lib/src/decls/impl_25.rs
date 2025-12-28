macro_rules! deps {
    () => {
        LibGitAdapter!();
        GitAdapter!();
        SubmoduleStat!();
        Result!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        # [cfg (feature = "git2_enabled")] impl GitAdapter for LibGitAdapter { fn list_submodules (& self , root_dir : & Path) -> Result < Vec < (String , PathBuf) > > { println ! ("[LibGitAdapter] Listing submodules in {:?}" , root_dir) ; let repo = Repository :: open (root_dir) . context (format ! ("Failed to open git repository at {:?}" , root_dir)) ? ; let mut submodules_info = Vec :: new () ; for submodule in repo . submodules () ? { let path = submodule . path () . to_path_buf () ; let url = submodule . url () . unwrap_or_default () . to_string () ; submodules_info . push ((url , path)) ; } Ok (submodules_info) } fn get_submodule_head_and_workdir_hash (& self , path : & Path) -> Result < SubmoduleStat > { println ! ("[LibGitAdapter] Getting submodule stat for {:?}" , path) ; let repo = Repository :: open (path) . context (format ! ("Failed to open git repository at {:?}" , path)) ? ; let head = repo . head () ? ; let head_oid = head . target () . context ("Failed to get HEAD OID") ? ; let head_commit = head_oid . to_string () ; let statuses = repo . statuses (None) ? ; let dirty = statuses . iter () . any (| s | s . status () != git2 :: Status :: empty ()) ; let workdir_hash = if dirty { format ! ("{}-dirty" , head_commit) } else { head_commit . clone () } ; Ok (SubmoduleStat { head_commit , workdir_hash , }) } fn as_any (& self) -> & dyn Any { self } }
    };
}

impl_25!()