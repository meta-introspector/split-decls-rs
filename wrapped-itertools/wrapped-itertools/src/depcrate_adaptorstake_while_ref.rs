// Generated macro for take_while_ref (function)
macro_rules! Depcrate_adaptorstake_while_ref {
() => {
// Module: crate::adaptors
// Provides: {"take_while_ref"}
// Dependencies: {}
# [doc = " Create a new `TakeWhileRef` from a reference to cloneable iterator."] pub fn take_while_ref < I , F > (iter : & mut I , f : F) -> TakeWhileRef < '_ , I , F > where I : Iterator + Clone , { TakeWhileRef { iter , f } }
};
}
