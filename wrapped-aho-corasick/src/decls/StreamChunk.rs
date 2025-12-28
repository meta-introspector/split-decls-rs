macro_rules! deps {
    () => {
        Match!();
    };
}

macro_rules! StreamChunk {
    () => {
        deps!();
        # [doc = " A single chunk yielded by the stream chunk iterator."] # [doc = ""] # [doc = " The `'r` lifetime refers to the lifetime of the stream chunk iterator."] # [cfg (feature = "std")] # [derive (Debug)] enum StreamChunk < 'r > { # [doc = " A chunk that does not contain any matches."] NonMatch { bytes : & 'r [u8] } , # [doc = " A chunk that precisely contains a match."] Match { bytes : & 'r [u8] , mat : Match } , }
    };
}

StreamChunk!();