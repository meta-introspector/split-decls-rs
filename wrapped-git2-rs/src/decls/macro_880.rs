macro_rules! deps {
    () => {
        RepositoryInitOptions!();
    };
}

macro_rules! macro_880 {
    () => {
        deps!();
        bitflags ! { # [doc = " Mode options for RepositoryInitOptions"] # [derive (Clone , Copy , Debug , Eq , PartialEq , PartialOrd , Ord , Hash)] pub struct RepositoryInitMode : u32 { # [doc = " Use permissions configured by umask - the default"] const SHARED_UMASK = raw :: GIT_REPOSITORY_INIT_SHARED_UMASK as u32 ; # [doc = " Use `--shared=group` behavior, chmod'ing the new repo to be"] # [doc = " group writable and \\\"g+sx\\\" for sticky group assignment"] const SHARED_GROUP = raw :: GIT_REPOSITORY_INIT_SHARED_GROUP as u32 ; # [doc = " Use `--shared=all` behavior, adding world readability."] const SHARED_ALL = raw :: GIT_REPOSITORY_INIT_SHARED_ALL as u32 ; } }
    };
}

macro_880!()