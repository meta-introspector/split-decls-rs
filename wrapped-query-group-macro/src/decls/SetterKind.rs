macro_rules! deps {
    () => {
        InputSetterWithDurability!();
        InputSetter!();
    };
}

macro_rules! SetterKind {
    () => {
        deps!();
        pub (crate) enum SetterKind { Plain (InputSetter) , WithDurability (InputSetterWithDurability) , }
    };
}

SetterKind!()