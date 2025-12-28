macro_rules! deps {
    () => {
        MacroCallId!();
    };
}

macro_rules! impl_258 {
    () => {
        deps!();
        impl From < span :: MacroCallId > for MacroCallId { # [inline] fn from (value : span :: MacroCallId) -> Self { MacroCallId :: from_id (value . 0) } }
    };
}

impl_258!();