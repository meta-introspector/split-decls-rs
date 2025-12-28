macro_rules! deps {
    () => {
        Tree!();
        Header!();
        Kind!();
    };
}

macro_rules! to_header {
    () => {
        deps!();
        fn to_header (kind : gix_object :: Kind) -> Header { use gix_object :: Kind :: * ; match kind { Tree => Header :: Tree , Blob => Header :: Blob , Commit => Header :: Commit , Tag => Header :: Tag , } }
    };
}

to_header!()