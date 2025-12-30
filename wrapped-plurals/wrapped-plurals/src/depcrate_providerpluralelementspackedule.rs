// Generated macro for PluralElementsPackedULE (struct)
macro_rules! Depcrate_providerPluralElementsPackedULE {
() => {
// Module: crate::provider
// Provides: {"PluralElementsPackedULE"}
// Dependencies: {}
# [doc = " A bitpacked DST for [`PluralElements`]."] # [doc = ""] # [doc = " Can be put in a [`Cow`] or a [`VarZeroSlice`]."] # [derive (PartialEq , Eq)] # [repr (transparent)] pub struct PluralElementsPackedULE < V : VarULE + ? Sized > { _v : PhantomData < V > , # [doc = " Invariant Representation:"] # [doc = ""] # [doc = " First byte: `d...mmmm`"] # [doc = " - `d` = 0 if singleton, 1 if a map"] # [doc = " - `...` = padding, should be 0"] # [doc = " - `mmmm` = [`FourBitMetadata`] for the default value"] # [doc = ""] # [doc = " If d is 0:"] # [doc = " - Remainder: the default (plural \"other\") value `V`"] # [doc = ""] # [doc = " If d is 1:"] # [doc = " - Second byte: L = the length of `V`"] # [doc = " - Bytes 2..(2+L): the default (plural \"other\") value `V`"] # [doc = " - Remainder: [`PluralElementsTupleSliceVarULE`]"] bytes : [u8] , }
};
}
