// Generated macro for impl_440 (impl)
macro_rules! Depcrate_sync_muteximpl_440 {
() => {
// Module: crate::sync::mutex
// Provides: {"impl_440"}
// Dependencies: {}
impl < T > Mutex < T > { # [doc = " Creates a new mutex in an unlocked state ready for use."] pub fn new (data : T) -> Mutex < T > { Mutex { data : std :: sync :: Mutex :: new (data) , object : rt :: Mutex :: new (true) , } } # [doc = " Consumes this mutex, returning the underlying data."] pub fn into_inner (self) -> LockResult < T > { Ok (self . data . into_inner () . unwrap ()) } }
};
}
