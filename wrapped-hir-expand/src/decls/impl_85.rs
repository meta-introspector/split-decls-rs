macro_rules! deps {
    () => {
        InRealFile!();
        InFile!();
    };
}

macro_rules! impl_85 {
    () => {
        deps!();
        impl < T > From < InRealFile < T > > for InFile < T > { fn from (InRealFile { file_id , value } : InRealFile < T >) -> Self { InFile { file_id : file_id . into () , value } } }
    };
}

impl_85!()