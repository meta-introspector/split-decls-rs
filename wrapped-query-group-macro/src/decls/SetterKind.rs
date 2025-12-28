macro_rules! deps {
    () => {
        InputSetter!();
        InputSetterWithDurability!();
    };
}

macro_rules! SetterKind {
    () => {
        deps!();
        pub (crate) enum SetterKind { Plain (InputSetter) , WithDurability (InputSetterWithDurability) , }
    };
}

SetterKind!();