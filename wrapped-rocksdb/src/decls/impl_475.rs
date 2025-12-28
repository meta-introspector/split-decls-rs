macro_rules! deps {
    () => {
        WriteBatchWithTransaction!();
    };
}

macro_rules! impl_475 {
    () => {
        deps!();
        impl < const TRANSACTION : bool > Default for WriteBatchWithTransaction < TRANSACTION > { fn default () -> Self { Self :: new () } }
    };
}

impl_475!()