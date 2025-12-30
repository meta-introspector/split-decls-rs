// Generated macro for _impl_internable (macro)
macro_rules! Depcrate_impl_internable {
() => {
// Module: crate
// Provides: {"_impl_internable"}
// Dependencies: {}
# [doc = " Implements `Internable` for a given list of types, making them usable with `Interned`."] # [macro_export] # [doc (hidden)] macro_rules ! _impl_internable { ($ ($ t : path) ,+ $ (,) ?) => { $ (impl $ crate :: Internable for $ t { fn storage () -> &'static $ crate :: InternStorage < Self > { static STORAGE : $ crate :: InternStorage <$ t > = $ crate :: InternStorage :: new () ; & STORAGE } }) + } ; }
};
}
