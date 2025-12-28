macro_rules! deps {
    () => {
        ChangeRef!();
    };
}

macro_rules! Change {
    () => {
        deps!();
        # [doc = " The fully-owned version of [`ChangeRef`]."] pub type Change = ChangeRef < 'static , 'static > ;
    };
}

Change!();