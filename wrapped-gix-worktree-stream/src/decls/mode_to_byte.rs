macro_rules! mode_to_byte {
    () => {
        fn mode_to_byte (m : gix_object :: tree :: EntryMode) -> u8 { use gix_object :: tree :: EntryKind :: * ; match m . kind () { Tree => 0 , Blob => 1 , BlobExecutable => 2 , Link => 3 , Commit => 4 , } }
    };
}

mode_to_byte!()