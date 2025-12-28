macro_rules! deps {
    () => {
        WriteBatchWithTransaction!();
    };
}

macro_rules! WriteBatch {
    () => {
        deps!();
        # [doc = " A type alias to keep compatibility. See [`WriteBatchWithTransaction`] for details"] pub type WriteBatch = WriteBatchWithTransaction < false > ;
    };
}

WriteBatch!()