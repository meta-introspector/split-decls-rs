macro_rules! deps {
    () => {
        Control!();
    };
}

macro_rules! impl_37 {
    () => {
        deps!();
        # [doc = " The default is `Continue`."] impl < B > Default for Control < B > { fn default () -> Self { Control :: Continue } }
    };
}

impl_37!();