// Generated macro for IterMut (struct)
macro_rules! Depcrate_dequeIterMut {
() => {
// Module: crate::deque
// Provides: {"IterMut"}
// Dependencies: {}
# [doc = " Iterator over the contents of a [`Deque`]"] pub struct IterMut < 'a , T > { inner : core :: iter :: Chain < core :: slice :: IterMut < 'a , T > , core :: slice :: IterMut < 'a , T > > , }
};
}
