macro_rules! deps {
    () => {
        GroupInfoPatternNames!();
    };
}

macro_rules! impl_630 {
    () => {
        deps!();
        impl GroupInfoPatternNames < 'static > { fn empty () -> GroupInfoPatternNames < 'static > { GroupInfoPatternNames { it : [] . iter () } } }
    };
}

impl_630!()