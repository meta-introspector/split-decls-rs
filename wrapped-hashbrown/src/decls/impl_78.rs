macro_rules! deps {
    () => {
        RawTableInner!();
        RawIter!();
    };
}

macro_rules! impl_78 {
    () => {
        deps!();
        impl < T > Default for RawIter < T > { # [cfg_attr (feature = "inline-more" , inline)] fn default () -> Self { unsafe { RawTableInner :: NEW . iter () } } }
    };
}

impl_78!();