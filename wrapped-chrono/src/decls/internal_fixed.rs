macro_rules! deps {
    () => {
        InternalInternal!();
        Item!();
        InternalFixed!();
        Fixed!();
    };
}

macro_rules! internal_fixed {
    () => {
        deps!();
        const fn internal_fixed (val : InternalInternal) -> Item < 'static > { Item :: Fixed (Fixed :: Internal (InternalFixed { val })) }
    };
}

internal_fixed!();