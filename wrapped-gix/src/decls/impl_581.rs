macro_rules! deps {
    () => {
        Any!();
        Author!();
        Tree!();
    };
}

macro_rules! impl_581 {
    () => {
        deps!();
        impl Author { # [doc = " The `author.name` key."] pub const NAME : keys :: Any = keys :: Any :: new ("name" , & config :: Tree :: AUTHOR) . with_fallback (& gitoxide :: Author :: NAME_FALLBACK) ; # [doc = " The `author.email` key."] pub const EMAIL : keys :: Any = keys :: Any :: new ("email" , & config :: Tree :: AUTHOR) . with_fallback (& gitoxide :: Author :: EMAIL_FALLBACK) ; }
    };
}

impl_581!();