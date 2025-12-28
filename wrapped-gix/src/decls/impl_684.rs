macro_rules! deps {
    () => {
        Tree!();
        Path!();
        String!();
        Mailmap!();
    };
}

macro_rules! impl_684 {
    () => {
        deps!();
        impl Mailmap { # [doc = " The `mailmap.blob` key"] pub const BLOB : keys :: String = keys :: String :: new_string ("blob" , & Tree :: MAILMAP) ; # [doc = " The `mailmap.file` key"] pub const FILE : keys :: Path = keys :: Path :: new_path ("file" , & Tree :: MAILMAP) ; }
    };
}

impl_684!();