macro_rules! deps {
    () => {
        MacroCallId!();
    };
}

macro_rules! impl_259 {
    () => {
        deps!();
        impl From < MacroCallId > for span :: MacroCallId { # [inline] fn from (value : MacroCallId) -> span :: MacroCallId { span :: MacroCallId (value . as_id ()) } }
    };
}

impl_259!()