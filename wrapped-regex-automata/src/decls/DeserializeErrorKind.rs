macro_rules! deps {
    () => {
        PatternID!();
        StateID!();
    };
}

macro_rules! DeserializeErrorKind {
    () => {
        deps!();
        # [derive (Debug)] enum DeserializeErrorKind { Generic { msg : & 'static str } , BufferTooSmall { what : & 'static str } , InvalidUsize { what : & 'static str } , VersionMismatch { expected : u32 , found : u32 } , EndianMismatch { expected : u32 , found : u32 } , AlignmentMismatch { alignment : usize , address : usize } , LabelMismatch { expected : & 'static str } , ArithmeticOverflow { what : & 'static str } , PatternID { err : PatternIDError , what : & 'static str } , StateID { err : StateIDError , what : & 'static str } , }
    };
}

DeserializeErrorKind!()