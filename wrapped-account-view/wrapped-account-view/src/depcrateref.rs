// Generated macro for Ref (struct)
macro_rules! DepcrateRef {
() => {
// Module: crate
// Provides: {"Ref"}
// Dependencies: {}
# [doc = " Reference to account data with checked borrow rules."] # [derive (Debug)] pub struct Ref < 'a , T : ? Sized > { value : NonNull < T > , state : NonNull < u8 > , # [doc = " The `value` raw pointer is only valid while the `&'a T` lives so we claim"] # [doc = " to hold a reference to it."] marker : PhantomData < & 'a T > , }
};
}
