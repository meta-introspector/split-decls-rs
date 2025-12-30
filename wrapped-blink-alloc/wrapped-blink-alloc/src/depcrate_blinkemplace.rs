// Generated macro for Emplace (struct)
macro_rules! Depcrate_blinkEmplace {
() => {
// Module: crate::blink
// Provides: {"Emplace"}
// Dependencies: {}
# [doc = " Provides interface for emplacing values."] # [doc = " Created by [`Blink::emplace`], [`Blink::emplace_no_drop`]"] # [doc = " and [`Blink::emplace_unchecked`]."] pub struct Emplace < 'a , A , T , R = & 'a mut T , S = & 'a mut [T] > { blink : & 'a Blink < A > , no_drop : bool , marker : PhantomData < fn (T) -> (R , S) > , }
};
}
