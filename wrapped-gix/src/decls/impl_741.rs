macro_rules! deps {
    () => {
        Renames!();
        Tree!();
        Status!();
        ShowUntrackedFiles!();
        UnsignedInteger!();
    };
}

macro_rules! impl_741 {
    () => {
        deps!();
        impl Status { # [doc = " The `status.showUntrackedFiles` key"] pub const SHOW_UNTRACKED_FILES : ShowUntrackedFiles = ShowUntrackedFiles :: new_with_validate ("showUntrackedFiles" , & config :: Tree :: STATUS , validate :: ShowUntrackedFiles ,) ; # [doc = " The `status.renameLimit` key."] pub const RENAME_LIMIT : keys :: UnsignedInteger = keys :: UnsignedInteger :: new_unsigned_integer ("renameLimit" , & config :: Tree :: MERGE ,) . with_note ("The limit is actually squared, so 1000 stands for up to 1 million diffs if fuzzy rename tracking is enabled" ,) ; # [doc = " The `status.renames` key."] pub const RENAMES : super :: diff :: Renames = super :: diff :: Renames :: new_renames ("renames" , & config :: Tree :: MERGE) ; }
    };
}

impl_741!();