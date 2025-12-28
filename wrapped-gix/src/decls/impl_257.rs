macro_rules! deps {
    () => {
        Platform!();
        Iter!();
        Error!();
    };
}

macro_rules! impl_257 {
    () => {
        deps!();
        impl < 'repo > Platform < 'repo > { # [doc = " Return an iterator over all references in the repository, excluding"] # [doc = " pseudo references."] # [doc = ""] # [doc = " Even broken or otherwise unparsable or inaccessible references are returned and have to be handled by the caller on a"] # [doc = " case by case basis."] pub fn all (& self) -> Result < Iter < '_ , 'repo > , init :: Error > { Ok (Iter :: new (self . repo , self . platform . all () ?)) } # [doc = " Return an iterator over all references that match the given `prefix`."] # [doc = ""] # [doc = " These are of the form `refs/heads/` or `refs/remotes/origin`, and must not contain relative paths components like `.` or `..`."] pub fn prefixed < 'a > (& self , prefix : impl TryInto < & 'a RelativePath , Error = gix_path :: relative_path :: Error > ,) -> Result < Iter < '_ , 'repo > , init :: Error > { Ok (Iter :: new (self . repo , self . platform . prefixed (prefix . try_into () ?) ?)) } # [doc = " Return an iterator over all references that are tags."] # [doc = ""] # [doc = " They are all prefixed with `refs/tags`."] pub fn tags (& self) -> Result < Iter < '_ , 'repo > , init :: Error > { Ok (Iter :: new (self . repo , self . platform . prefixed (b"refs/tags/" . try_into () ?) ?)) } # [doc = " Return an iterator over all local branches."] # [doc = ""] # [doc = " They are all prefixed with `refs/heads`."] pub fn local_branches (& self) -> Result < Iter < '_ , 'repo > , init :: Error > { Ok (Iter :: new (self . repo , self . platform . prefixed (b"refs/heads/" . try_into () ?) ? ,)) } # [doc = " Return an iterator over all local pseudo references."] pub fn pseudo (& self) -> Result < Iter < '_ , 'repo > , init :: Error > { Ok (Iter :: new (self . repo , self . platform . pseudo () ?)) } # [doc = " Return an iterator over all remote branches."] # [doc = ""] # [doc = " They are all prefixed with `refs/remotes`."] pub fn remote_branches (& self) -> Result < Iter < '_ , 'repo > , init :: Error > { Ok (Iter :: new (self . repo , self . platform . prefixed (b"refs/remotes/" . try_into () ?) ? ,)) } }
    };
}

impl_257!();