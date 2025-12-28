macro_rules! deps {
    () => {
        Safe!();
        Tree!();
        Any!();
    };
}

macro_rules! impl_729 {
    () => {
        deps!();
        impl Safe { # [doc = " The `safe.directory` key"] pub const DIRECTORY : keys :: Any = keys :: Any :: new ("directory" , & config :: Tree :: SAFE) ; }
    };
}

impl_729!()