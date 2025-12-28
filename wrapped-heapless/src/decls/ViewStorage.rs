macro_rules! ViewStorage {
    () => {
        # [doc = " Implementation of [`StringStorage`] that stores the data in an unsized slice."] pub type ViewStorage = ViewVecStorage < u8 > ;
    };
}

ViewStorage!();