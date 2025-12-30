// Generated macro for Iter (struct)
macro_rules! Depcrate_dequeIter {
() => {
// Module: crate::deque
// Provides: {"Iter"}
// Dependencies: {}
# [doc = " Iterator over the contents of a [`Deque`]"] pub struct Iter < 'a , T > { inner : core :: iter :: Chain < core :: slice :: Iter < 'a , T > , core :: slice :: Iter < 'a , T > > , }
};
}
