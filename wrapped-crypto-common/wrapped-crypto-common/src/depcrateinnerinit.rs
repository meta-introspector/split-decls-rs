// Generated macro for InnerInit (trait)
macro_rules! DepcrateInnerInit {
() => {
// Module: crate
// Provides: {"InnerInit"}
// Dependencies: {}
# [doc = " Types which can be initialized from another type (usually block ciphers)."] # [doc = ""] # [doc = " Usually used for initializing types from block ciphers."] pub trait InnerInit : InnerUser + Sized { # [doc = " Initialize value from the `inner`."] fn inner_init (inner : Self :: Inner) -> Self ; }
};
}
