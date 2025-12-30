// Generated macro for Drain (struct)
macro_rules! DepcrateDrain {
() => {
// Module: crate
// Provides: {"Drain"}
// Dependencies: {}
# [doc = " An iterator that removes the items from a `SmallVec` and yields them by value."] # [doc = ""] # [doc = " Returned from [`SmallVec::drain`][1]."] # [doc = ""] # [doc = " [1]: struct.SmallVec.html#method.drain"] pub struct Drain < 'a , T : 'a , const N : usize > { tail_start : usize , tail_len : usize , iter : core :: slice :: Iter < 'a , T > , vec : core :: ptr :: NonNull < SmallVec < T , N > > , }
};
}
