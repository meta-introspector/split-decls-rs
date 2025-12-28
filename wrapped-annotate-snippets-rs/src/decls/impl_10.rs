macro_rules! deps {
    () => {
        Stylesheet!();
        LevelInner!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl LevelInner { pub (crate) fn style (self , stylesheet : & Stylesheet) -> Style { match self { LevelInner :: Error => stylesheet . error , LevelInner :: Warning => stylesheet . warning , LevelInner :: Info => stylesheet . info , LevelInner :: Note => stylesheet . note , LevelInner :: Help => stylesheet . help , } } }
    };
}

impl_10!();