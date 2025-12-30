// Generated macro for IndexVec (struct)
macro_rules! Depcrate_vecIndexVec {
() => {
// Module: crate::vec
// Provides: {"IndexVec"}
// Dependencies: {}
# [doc = " An owned contiguous collection of `T`s, indexed by `I` rather than by `usize`."] # [doc = ""] # [doc = " ## Why use this instead of a `Vec`?"] # [doc = ""] # [doc = " An `IndexVec` allows element access only via a specific associated index type, meaning that"] # [doc = " trying to use the wrong index type (possibly accessing an invalid element) will fail at"] # [doc = " compile time."] # [doc = ""] # [doc = " It also documents what the index is indexing: in a `HashMap<usize, Something>` it's not"] # [doc = " immediately clear what the `usize` means, while a `HashMap<FieldIdx, Something>` makes it obvious."] # [doc = ""] # [doc = " ```compile_fail"] # [doc = " use rustc_index::{Idx, IndexVec};"] # [doc = ""] # [doc = " fn f<I1: Idx, I2: Idx>(vec1: IndexVec<I1, u8>, idx1: I1, idx2: I2) {"] # [doc = "   &vec1[idx1]; // Ok"] # [doc = "   &vec1[idx2]; // Compile error!"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " While it's possible to use `u32` or `usize` directly for `I`,"] # [doc = " you almost certainly want to use a [`newtype_index!`]-generated type instead."] # [doc = ""] # [doc = " This allows to index the IndexVec with the new index type."] # [doc = ""] # [doc = " [`newtype_index!`]: ../macro.newtype_index.html"] # [derive (Clone , PartialEq , Eq , Hash)] # [repr (transparent)] pub struct IndexVec < I : Idx , T > { pub raw : Vec < T > , _marker : PhantomData < fn (& I) > , }
};
}
