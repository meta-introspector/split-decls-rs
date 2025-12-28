macro_rules! deps {
    () => {
        Change!();
        Mode!();
    };
}

macro_rules! impl_85 {
    () => {
        deps!();
        impl Change { # [doc = " Applies this change to `mode` and returns the changed one."] pub fn apply (self , mode : Mode) -> Mode { match self { Change :: Type { new_mode } => new_mode , Change :: ExecutableBit => match mode { Mode :: FILE => Mode :: FILE_EXECUTABLE , Mode :: FILE_EXECUTABLE => Mode :: FILE , _ => unreachable ! ("invalid mode change: can't flip executable bit of {mode:?}") , } , } } }
    };
}

impl_85!()