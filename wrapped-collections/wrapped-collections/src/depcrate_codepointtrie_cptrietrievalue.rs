// Generated macro for TrieValue (trait)
macro_rules! Depcrate_codepointtrie_cptrieTrieValue {
() => {
// Module: crate::codepointtrie::cptrie
// Provides: {"TrieValue"}
// Dependencies: {}
# [doc = " A trait representing the values stored in the data array of a [`CodePointTrie`]."] # [doc = " This trait is used as a type parameter in constructing a `CodePointTrie`."] # [doc = ""] # [doc = " This trait can be implemented on anything that can be represented as a u32s worth of data."] pub trait TrieValue : Copy + Eq + PartialEq + zerovec :: ule :: AsULE + 'static { # [doc = " Last-resort fallback value to return if we cannot read data from the trie."] # [doc = ""] # [doc = " In most cases, the error value is read from the last element of the `data` array,"] # [doc = " this value is used for empty codepointtrie arrays"] # [doc = " Error type when converting from a u32 to this `TrieValue`."] type TryFromU32Error : Display ; # [doc = " A parsing function that is primarily motivated by deserialization contexts."] # [doc = " When the serialization type width is smaller than 32 bits, then it is expected"] # [doc = " that the call site will widen the value to a `u32` first."] fn try_from_u32 (i : u32) -> Result < Self , Self :: TryFromU32Error > ; # [doc = " A method for converting back to a `u32` that can roundtrip through"] # [doc = " [`Self::try_from_u32()`]. The default implementation of this trait"] # [doc = " method panics in debug mode and returns 0 in release mode."] # [doc = ""] # [doc = " This method is allowed to have GIGO behavior when fed a value that has"] # [doc = " no corresponding `u32` (since such values cannot be stored in the trie)"] fn to_u32 (self) -> u32 ; }
};
}
