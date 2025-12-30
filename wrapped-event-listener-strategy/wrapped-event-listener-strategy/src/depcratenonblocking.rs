// Generated macro for NonBlocking (struct)
macro_rules! DepcrateNonBlocking {
() => {
// Module: crate
// Provides: {"NonBlocking"}
// Dependencies: {}
# [doc = " A strategy that uses polling to efficiently wait for an event."] # [derive (Debug , Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash , Default)] pub struct NonBlocking < 'a > { # [doc = " The type `&'a mut &'a T` is invariant over `'a`, like `Context` is."] # [doc = ""] # [doc = " We used to just use `Context` here, but then `Context` became `!Send`"] # [doc = " and `!Sync`, making all of the futures that use this type `!Send` and"] # [doc = " `!Sync` as well. So we just take the lifetime invariance and none of"] # [doc = " the downsides."] _marker : PhantomData < & 'a mut & 'a () > , }
};
}
