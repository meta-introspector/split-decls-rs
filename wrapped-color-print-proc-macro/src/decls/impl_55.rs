macro_rules! deps {
    () => {
        ColorKind!();
        Color!();
        Change!();
    };
}

macro_rules! impl_55 {
    () => {
        deps!();
        impl ColorKind { pub fn to_change (& self , color : Color) -> Change { match self { Self :: Foreground => Change :: Foreground (color) , Self :: Background => Change :: Background (color) , } } }
    };
}

impl_55!();