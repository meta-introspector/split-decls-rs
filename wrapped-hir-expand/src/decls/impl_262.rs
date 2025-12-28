macro_rules! deps {
    () => {
        MacroCallId!();
        HirFileId!();
    };
}

macro_rules! impl_262 {
    () => {
        deps!();
        impl From < MacroCallId > for HirFileId { # [inline] fn from (file_id : MacroCallId) -> Self { HirFileId :: MacroFile (file_id) } }
    };
}

impl_262!();