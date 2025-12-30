// Generated macro for Resource (enum)
macro_rules! Depcrate_bundleResource {
() => {
// Module: crate::bundle
// Provides: {"Resource"}
// Dependencies: {}
# [doc = " A data resource within a [`ResourceBundle`]."] # [derive (Debug)] # [non_exhaustive] pub enum Resource < 'a > { # [doc = " A well-formed UTF-8 string."] String (Cow < 'a , str >) , # [doc = " A heterogeneous list of resources, ordered by insertion."] Array (Vec < Resource < 'a > >) , # [doc = " A set of key-resource pairs, sorted lexically by key."] Table (Table < 'a >) , # [doc = " A slice of arbitrary binary data."] Binary (Cow < 'a , [u8] >) , # [doc = " A 28-bit integer."] # [doc = ""] # [doc = " May be interpreted as either signed or unsigned depending on consumer"] # [doc = " expectations. See [`Int28`] for further details."] Integer (Int28) , # [doc = " A list of 32-bit integers, ordered by insertion."] IntVector (Vec < u32 >) , }
};
}
