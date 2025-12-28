macro_rules! deps {
    () => {
        Committer!();
        Any!();
        Tree!();
    };
}

macro_rules! impl_604 {
    () => {
        deps!();
        impl Committer { # [doc = " The `committer.name` key."] pub const NAME : keys :: Any = keys :: Any :: new ("name" , & config :: Tree :: COMMITTER) . with_fallback (& gitoxide :: Committer :: NAME_FALLBACK) ; # [doc = " The `committer.email` key."] pub const EMAIL : keys :: Any = keys :: Any :: new ("email" , & config :: Tree :: COMMITTER) . with_fallback (& gitoxide :: Committer :: EMAIL_FALLBACK) ; }
    };
}

impl_604!();