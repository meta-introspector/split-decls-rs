// Generated macro for LoaderReader (type)
macro_rules! Depcrate_loaderLoaderReader {
() => {
// Module: crate::loader
// Provides: {"LoaderReader"}
// Dependencies: {}
# [doc = " The type used by [`Loader`] for reading DWARF data."] # [doc = ""] # [doc = " This is used in the return types of the methods of [`Loader`]."] pub type LoaderReader < 'a > = gimli :: EndianSlice < 'a , gimli :: RunTimeEndian > ;
};
}
