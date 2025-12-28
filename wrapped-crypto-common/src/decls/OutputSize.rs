macro_rules! deps {
    () => {
        OutputSizeUser!();
    };
}

macro_rules! OutputSize {
    () => {
        deps!();
        # [doc = " Alias for the output size of [`OutputSizeUser`] implementors."] pub type OutputSize < T > = < T as OutputSizeUser > :: OutputSize ;
    };
}

OutputSize!()