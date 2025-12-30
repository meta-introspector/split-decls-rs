// Generated macro for impl_160 (impl)
macro_rules! Depcrate_distr_sliceimpl_160 {
() => {
// Module: crate::distr::slice
// Provides: {"impl_160"}
// Dependencies: {}
impl < 'a , T > Choose < 'a , T > { # [doc = " Create a new `Choose` instance which samples uniformly from the slice."] # [doc = ""] # [doc = " Returns error [`Empty`] if the slice is empty."] pub fn new (slice : & 'a [T]) -> Result < Self , Empty > { let num_choices = NonZeroUsize :: new (slice . len ()) . ok_or (Empty) ? ; Ok (Self { slice , range : UniformUsize :: new (0 , num_choices . get ()) . unwrap () , num_choices , }) } # [doc = " Returns the count of choices in this distribution"] pub fn num_choices (& self) -> NonZeroUsize { self . num_choices } }
};
}
