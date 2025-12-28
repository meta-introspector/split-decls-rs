macro_rules! byte_to_mode {
    () => {
        fn byte_to_mode (b : u8) -> gix_object :: tree :: EntryMode { use gix_object :: tree :: EntryKind :: * ; match b { 0 => Tree , 1 => Blob , 2 => BlobExecutable , 3 => Link , 4 => Commit , _ => unreachable ! ("BUG: we control the protocol") , } . into () }
    };
}

byte_to_mode!()