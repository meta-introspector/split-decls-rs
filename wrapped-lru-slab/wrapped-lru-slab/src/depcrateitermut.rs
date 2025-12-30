// Generated macro for IterMut (struct)
macro_rules! DepcrateIterMut {
() => {
// Module: crate
// Provides: {"IterMut"}
// Dependencies: {}
# [doc = " Iterator over mutable elements of an [`LruSlab`], from most to least recently used"] pub struct IterMut < 'a , T > { slots : * mut Slot < T > , state : IterState , _marker : PhantomData < & 'a mut [Slot < T >] > , }
};
}
