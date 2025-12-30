// Generated macro for impl_26 (impl)
macro_rules! Depcrate_mapimpl_26 {
() => {
// Module: crate::map
// Provides: {"impl_26"}
// Dependencies: {}
impl < K , V , S > LiteMap < K , V , S > where S : StoreMut < K , V > , { # [doc = " Construct a new [`LiteMap`] with a given capacity"] pub fn with_capacity (capacity : usize) -> Self { Self { values : S :: lm_with_capacity (capacity) , _key_type : PhantomData , _value_type : PhantomData , } } # [doc = " Remove all elements from the [`LiteMap`]"] pub fn clear (& mut self) { self . values . lm_clear () } # [doc = " Reserve capacity for `additional` more elements to be inserted into"] # [doc = " the [`LiteMap`] to avoid frequent reallocations."] # [doc = ""] # [doc = " See [`Vec::reserve()`] for more information."] # [doc = ""] # [doc = " [`Vec::reserve()`]: alloc::vec::Vec::reserve"] pub fn reserve (& mut self , additional : usize) { self . values . lm_reserve (additional) } }
};
}
