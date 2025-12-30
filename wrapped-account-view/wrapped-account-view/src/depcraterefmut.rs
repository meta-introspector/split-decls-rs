// Generated macro for RefMut (struct)
macro_rules! DepcrateRefMut {
() => {
// Module: crate
// Provides: {"RefMut"}
// Dependencies: {}
# [doc = " Mutable reference to account data with checked borrow rules."] # [derive (Debug)] pub struct RefMut < 'a , T : ? Sized > { value : NonNull < T > , state : NonNull < u8 > , # [doc = " The `value` raw pointer is only valid while the `&'a T` lives so we claim"] # [doc = " to hold a reference to it."] marker : PhantomData < & 'a mut T > , }
};
}
