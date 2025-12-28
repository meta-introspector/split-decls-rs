macro_rules! deps {
    () => {
        DFA!();
    };
}

macro_rules! AlignAs {
    () => {
        deps!();
        # [doc = " A hack to align a smaller type `B` with a bigger type `T`."] # [doc = ""] # [doc = " The usual use of this is with `B = [u8]` and `T = u32`. That is,"] # [doc = " it permits aligning a sequence of bytes on a 4-byte boundary. This"] # [doc = " is useful in contexts where one wants to embed a serialized [dense"] # [doc = " DFA](crate::dfa::dense::DFA) into a Rust a program while guaranteeing the"] # [doc = " alignment required for the DFA."] # [doc = ""] # [doc = " See [`dense::DFA::from_bytes`](crate::dfa::dense::DFA::from_bytes) for an"] # [doc = " example of how to use this type."] # [repr (C)] # [derive (Debug)] pub struct AlignAs < B : ? Sized , T > { # [doc = " A zero-sized field indicating the alignment we want."] pub _align : [T ; 0] , # [doc = " A possibly non-sized field containing a sequence of bytes."] pub bytes : B , }
    };
}

AlignAs!()