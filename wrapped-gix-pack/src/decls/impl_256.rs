macro_rules! deps {
    () => {
        SafetyCheck!();
    };
}

macro_rules! impl_256 {
    () => {
        deps!();
        impl SafetyCheck { pub (crate) fn file_checksum (& self) -> bool { matches ! (self , SafetyCheck :: All) } pub (crate) fn object_checksum (& self) -> bool { matches ! (self , SafetyCheck :: All | SafetyCheck :: SkipFileChecksumVerification) } pub (crate) fn fatal_decode_error (& self) -> bool { match self { SafetyCheck :: All | SafetyCheck :: SkipFileChecksumVerification | SafetyCheck :: SkipFileAndObjectChecksumVerification => true , SafetyCheck :: SkipFileAndObjectChecksumVerificationAndNoAbortOnDecodeError => false , } } }
    };
}

impl_256!()