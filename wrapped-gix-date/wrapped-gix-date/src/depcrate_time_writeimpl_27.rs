// Generated macro for impl_27 (impl)
macro_rules! Depcrate_time_writeimpl_27 {
() => {
// Module: crate::time::write
// Provides: {"impl_27"}
// Dependencies: {}
# [doc = " Serialize this instance as string, similar to what [`write_to()`](Self::write_to()) would do."] impl std :: fmt :: Display for Time { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { let mut buf = Vec :: with_capacity (Time :: MAX . size ()) ; self . write_to (& mut buf) . expect ("write to memory cannot fail") ; # [allow (unsafe_code)] let raw = unsafe { buf . to_str_unchecked () } ; f . write_str (raw) } }
};
}
