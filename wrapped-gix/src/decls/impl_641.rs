macro_rules! deps {
    () => {
        Tree!();
        Boolean!();
        Extensions!();
        ObjectFormat!();
    };
}

macro_rules! impl_641 {
    () => {
        deps!();
        impl Extensions { # [doc = " The `extensions.worktreeConfig` key."] pub const WORKTREE_CONFIG : keys :: Boolean = keys :: Boolean :: new_boolean ("worktreeConfig" , & config :: Tree :: EXTENSIONS) ; # [doc = " The `extensions.objectFormat` key."] pub const OBJECT_FORMAT : ObjectFormat = ObjectFormat :: new_with_validate ("objectFormat" , & config :: Tree :: EXTENSIONS , validate :: ObjectFormat) . with_note ("Support for SHA256 is prepared but not fully implemented yet. For now we abort when encountered" ,) ; }
    };
}

impl_641!()