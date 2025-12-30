// Generated macro for Iter (struct)
macro_rules! DepcrateIter {
() => {
// Module: crate
// Provides: {"Iter"}
// Dependencies: {}
# [doc = " An iterator over the entries of a `LruCache`."] # [doc = ""] # [doc = " This `struct` is created by the [`iter`] method on [`LruCache`][`LruCache`]. See its"] # [doc = " documentation for more."] # [doc = ""] # [doc = " [`iter`]: struct.LruCache.html#method.iter"] # [doc = " [`LruCache`]: struct.LruCache.html"] pub struct Iter < 'a , K : 'a , V : 'a > { len : usize , ptr : * const LruEntry < K , V > , end : * const LruEntry < K , V > , phantom : PhantomData < & 'a K > , }
};
}
