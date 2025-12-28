macro_rules! deps {
    () => {
        HirFileId!();
        MacroCallId!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        impl From < MacroCallId > for HirFileId { # [inline] fn from (file_id : MacroCallId) -> Self { HirFileId :: MacroFile (file_id) } }
    };
}

impl_56!()