macro_rules! deps {
    () => {
        Any!();
        Tree!();
        Safe!();
    };
}

macro_rules! impl_729 {
    () => {
        deps!();
        impl Safe { # [doc = " The `safe.directory` key"] pub const DIRECTORY : keys :: Any = keys :: Any :: new ("directory" , & config :: Tree :: SAFE) ; }
    };
}

impl_729!();