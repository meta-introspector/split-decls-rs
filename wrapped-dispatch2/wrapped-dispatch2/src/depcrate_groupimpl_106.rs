// Generated macro for impl_106 (impl)
macro_rules! Depcrate_groupimpl_106 {
() => {
// Module: crate::group
// Provides: {"impl_106"}
// Dependencies: {}
impl DispatchGroup { # [doc = " Submit a function to a [`DispatchQueue`] and associates it with the [`DispatchGroup`]."] pub fn exec_async < F > (& self , queue : & DispatchQueue , work : F) where F : Send + FnOnce () + 'static , { let work_boxed = Box :: into_raw (Box :: new (work)) . cast :: < c_void > () ; unsafe { Self :: exec_async_f (self , queue , work_boxed , function_wrapper :: < F >) } ; } # [doc = " Wait synchronously for the previously submitted functions to finish."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " Return [WaitError::Timeout] in case of timeout."] pub fn wait (& self , timeout : DispatchTime) -> Result < () , WaitError > { let result = dispatch_group_wait (self , timeout) ; match result { 0 => Ok (()) , _ => Err (WaitError :: Timeout) , } } # [doc = " Schedule a function to be submitted to a [`DispatchQueue`] when a group of previously submitted functions have completed."] pub fn notify < F > (& self , queue : & DispatchQueue , work : F) where F : Send + FnOnce () , { let work_boxed = Box :: into_raw (Box :: new (work)) . cast :: < c_void > () ; unsafe { Self :: notify_f (self , queue , work_boxed , function_wrapper :: < F >) ; } } # [doc = " Explicitly indicates that the function has entered the [`DispatchGroup`]."] pub fn enter (& self) -> DispatchGroupGuard { unsafe { dispatch_group_enter (self) } ; DispatchGroupGuard (self . retain ()) } }
};
}
