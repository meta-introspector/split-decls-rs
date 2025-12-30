// Generated macro for IterMut (struct)
macro_rules! DepcrateIterMut {
() => {
// Module: crate
// Provides: {"IterMut"}
// Dependencies: {}
# [doc = " An iterator over mutables entries of a `LruCache`."] # [doc = ""] # [doc = " This `struct` is created by the [`iter_mut`] method on [`LruCache`][`LruCache`]. See its"] # [doc = " documentation for more."] # [doc = ""] # [doc = " [`iter_mut`]: struct.LruCache.html#method.iter_mut"] # [doc = " [`LruCache`]: struct.LruCache.html"] pub struct IterMut < 'a , K : 'a , V : 'a > { len : usize , ptr : * mut LruEntry < K , V > , end : * mut LruEntry < K , V > , phantom : PhantomData < & 'a K > , }
};
}
