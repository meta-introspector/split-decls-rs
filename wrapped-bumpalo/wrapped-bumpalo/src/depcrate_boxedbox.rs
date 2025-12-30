// Generated macro for Box (struct)
macro_rules! Depcrate_boxedBox {
() => {
// Module: crate::boxed
// Provides: {"Box"}
// Dependencies: {}
# [doc = " An owned pointer to a bump-allocated `T` value, that runs `Drop`"] # [doc = " implementations."] # [doc = ""] # [doc = " See the [module-level documentation][crate::boxed] for more details."] # [repr (transparent)] pub struct Box < 'a , T : ? Sized > (& 'a mut T) ;
};
}
