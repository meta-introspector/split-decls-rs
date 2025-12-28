macro_rules! deps {
    () => {
        IndexThreads!();
        Index!();
        Boolean!();
        Tree!();
    };
}

macro_rules! impl_673 {
    () => {
        deps!();
        impl Index { # [doc = " The `index.threads` key."] pub const THREADS : IndexThreads = IndexThreads :: new_with_validate ("threads" , & config :: Tree :: INDEX , validate :: IndexThreads) ; # [doc = " The `index.skipHash` key."] pub const SKIP_HASH : keys :: Boolean = keys :: Boolean :: new_boolean ("skipHash" , & config :: Tree :: INDEX) . with_deviation ("also used to skip the hash when reading, even if a hash exists in the index file") ; }
    };
}

impl_673!()