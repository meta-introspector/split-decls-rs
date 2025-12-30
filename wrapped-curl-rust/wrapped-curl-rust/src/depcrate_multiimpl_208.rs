// Generated macro for impl_208 (impl)
macro_rules! Depcrate_multiimpl_208 {
() => {
// Module: crate::multi
// Provides: {"impl_208"}
// Dependencies: {}
# [cfg (feature = "poll_7_68_0")] impl MultiWaker { # [doc = " Creates a new MultiWaker handle."] fn new (raw : std :: sync :: Weak < RawMulti >) -> Self { Self { raw } } # [doc = " Wakes up a thread that is blocked in [Multi::poll]. This method can be"] # [doc = " invoked from any thread."] # [doc = ""] # [doc = " Will return an error if the RawMulti has already been dropped."] # [doc = ""] # [doc = " Requires libcurl 7.68.0 or later."] pub fn wakeup (& self) -> Result < () , MultiError > { if let Some (raw) = self . raw . upgrade () { unsafe { cvt (curl_sys :: curl_multi_wakeup (raw . handle)) } } else { Err (MultiError :: new (curl_sys :: CURLM_BAD_HANDLE)) } } }
};
}
