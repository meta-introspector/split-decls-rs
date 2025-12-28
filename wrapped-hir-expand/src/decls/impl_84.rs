macro_rules! deps {
    () => {
        InMacroFile!();
        InFile!();
    };
}

macro_rules! impl_84 {
    () => {
        deps!();
        impl < T > From < InMacroFile < T > > for InFile < T > { fn from (InMacroFile { file_id , value } : InMacroFile < T >) -> Self { InFile { file_id : file_id . into () , value } } }
    };
}

impl_84!()