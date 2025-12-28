macro_rules! deps {
    () => {
        DeserializeError!();
        DeserializeErrorKind!();
        PatternID!();
        StateID!();
    };
}

macro_rules! impl_807 {
    () => {
        deps!();
        impl core :: fmt :: Display for DeserializeError { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { use self :: DeserializeErrorKind :: * ; match self . 0 { Generic { msg } => write ! (f , "{msg}") , BufferTooSmall { what } => { write ! (f , "buffer is too small to read {what}") } InvalidUsize { what } => { write ! (f , "{what} is too big to fit in a usize") } VersionMismatch { expected , found } => write ! (f , "unsupported version: \
                 expected version {expected} but found version {found}" ,) , EndianMismatch { expected , found } => write ! (f , "endianness mismatch: expected 0x{expected:X} but \
                 got 0x{found:X}. (Are you trying to load an object \
                 serialized with a different endianness?)" ,) , AlignmentMismatch { alignment , address } => write ! (f , "alignment mismatch: slice starts at address 0x{address:X}, \
                 which is not aligned to a {alignment} byte boundary" ,) , LabelMismatch { expected } => write ! (f , "label mismatch: start of serialized object should \
                 contain a NUL terminated {expected:?} label, but a different \
                 label was found" ,) , ArithmeticOverflow { what } => { write ! (f , "arithmetic overflow for {what}") } PatternID { ref err , what } => { write ! (f , "failed to read pattern ID for {what}: {err}") } StateID { ref err , what } => { write ! (f , "failed to read state ID for {what}: {err}") } } } }
    };
}

impl_807!();