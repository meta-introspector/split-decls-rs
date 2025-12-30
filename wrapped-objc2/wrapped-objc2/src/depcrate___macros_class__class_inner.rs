// Generated macro for __class_inner (macro)
macro_rules! Depcrate___macros_class__class_inner {
() => {
// Module: crate::__macros::class
// Provides: {"__class_inner"}
// Dependencies: {}
# [doc (hidden)] # [macro_export] # [cfg (all (feature = "unstable-static-class" , feature = "gnustep-1-7"))] macro_rules ! __class_inner { ($ name : expr , $ _hash : expr) => { { extern "C" { # [link_name = $ crate :: __class_static_name ! ($ name)] static CLASS : $ crate :: runtime :: AnyClass ; } # [allow (unused_unsafe)] unsafe { $ crate :: __macros :: disallow_in_static (& CLASS) } } } ; }
};
}
