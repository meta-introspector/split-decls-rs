macro_rules! deps {
    () => {
        WriteBatchWithTransaction!();
    };
}

macro_rules! impl_477 {
    () => {
        deps!();
        unsafe impl < const TRANSACTION : bool > Send for WriteBatchWithTransaction < TRANSACTION > { }
    };
}

impl_477!()