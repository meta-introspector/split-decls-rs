// Generated macro for impl_128 (impl)
macro_rules! Depcrate_main_thread_boundimpl_128 {
() => {
// Module: crate::main_thread_bound
// Provides: {"impl_128"}
// Dependencies: {}
# [doc = " Helper functions for running [`run_on_main`]."] impl < T > MainThreadBound < T > { # [doc = " Access the item on the main thread."] # [doc = ""] # [doc = " See [`run_on_main`] for caveats."] # [inline] pub fn get_on_main < F , R > (& self , f : F) -> R where F : Send + FnOnce (& T) -> R , R : Send , { run_on_main (| mtm | f (self . get (mtm))) } # [doc = " Access the item mutably on the main thread."] # [doc = ""] # [doc = " See [`run_on_main`] for caveats."] # [inline] pub fn get_on_main_mut < F , R > (& mut self , f : F) -> R where F : Send + FnOnce (& mut T) -> R , R : Send , { run_on_main (| mtm | f (self . get_mut (mtm))) } }
};
}
