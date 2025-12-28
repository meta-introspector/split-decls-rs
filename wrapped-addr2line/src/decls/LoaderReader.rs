macro_rules! deps {
    () => {
        Loader!();
    };
}

macro_rules! LoaderReader {
    () => {
        deps!();
        # [doc = " The type used by [`Loader`] for reading DWARF data."] # [doc = ""] # [doc = " This is used in the return types of the methods of [`Loader`]."] pub type LoaderReader < 'a > = gimli :: EndianSlice < 'a , gimli :: RunTimeEndian > ;
    };
}

LoaderReader!();