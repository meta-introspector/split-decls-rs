macro_rules! deps {
    () => {
        DeserializeError!();
        PatternID!();
        DeserializeErrorKind!();
        StateID!();
    };
}

macro_rules! impl_805 {
    () => {
        deps!();
        impl DeserializeError { pub (crate) fn generic (msg : & 'static str) -> DeserializeError { DeserializeError (DeserializeErrorKind :: Generic { msg }) } pub (crate) fn buffer_too_small (what : & 'static str) -> DeserializeError { DeserializeError (DeserializeErrorKind :: BufferTooSmall { what }) } fn invalid_usize (what : & 'static str) -> DeserializeError { DeserializeError (DeserializeErrorKind :: InvalidUsize { what }) } fn version_mismatch (expected : u32 , found : u32) -> DeserializeError { DeserializeError (DeserializeErrorKind :: VersionMismatch { expected , found , }) } fn endian_mismatch (expected : u32 , found : u32) -> DeserializeError { DeserializeError (DeserializeErrorKind :: EndianMismatch { expected , found , }) } fn alignment_mismatch (alignment : usize , address : usize ,) -> DeserializeError { DeserializeError (DeserializeErrorKind :: AlignmentMismatch { alignment , address , }) } fn label_mismatch (expected : & 'static str) -> DeserializeError { DeserializeError (DeserializeErrorKind :: LabelMismatch { expected }) } fn arithmetic_overflow (what : & 'static str) -> DeserializeError { DeserializeError (DeserializeErrorKind :: ArithmeticOverflow { what }) } fn pattern_id_error (err : PatternIDError , what : & 'static str ,) -> DeserializeError { DeserializeError (DeserializeErrorKind :: PatternID { err , what }) } pub (crate) fn state_id_error (err : StateIDError , what : & 'static str ,) -> DeserializeError { DeserializeError (DeserializeErrorKind :: StateID { err , what }) } }
    };
}

impl_805!()