macro_rules! deps {
    () => {
        PunycodeEncodeError!();
        ProcessingError!();
    };
}

macro_rules! impl_72 {
    () => {
        deps!();
        impl From < crate :: punycode :: PunycodeEncodeError > for ProcessingError { fn from (_ : crate :: punycode :: PunycodeEncodeError) -> Self { unreachable ! ("Punycode overflows should not be possible due to PUNYCODE_ENCODE_MAX_INPUT_LENGTH") ; } }
    };
}

impl_72!();