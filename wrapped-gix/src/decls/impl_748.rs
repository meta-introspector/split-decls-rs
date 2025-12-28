macro_rules! deps {
    () => {
        Tree!();
        User!();
        Any!();
    };
}

macro_rules! impl_748 {
    () => {
        deps!();
        impl User { # [doc = " The `user.name` key"] pub const NAME : keys :: Any = keys :: Any :: new ("name" , & config :: Tree :: USER) ; # [doc = " The `user.email` key"] pub const EMAIL : keys :: Any = keys :: Any :: new ("email" , & config :: Tree :: USER) . with_fallback (& gitoxide :: User :: EMAIL_FALLBACK) ; }
    };
}

impl_748!()