// Generated macro for impl_7 (impl)
macro_rules! Depcrate_muteximpl_7 {
() => {
// Module: crate::mutex
// Provides: {"impl_7"}
// Dependencies: {}
impl < T : Default > Mutex < RefCell < T > > { # [doc = " Borrow the data and call [`RefCell::take`]"] # [doc = ""] # [doc = " This is equivalent to `self.borrow(cs).take()`"] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " This call could panic. See the documentation for [`RefCell::take`]"] # [doc = " for more details."] # [inline] # [track_caller] pub fn take < 'cs > (& 'cs self , cs : CriticalSection < 'cs >) -> T { self . borrow (cs) . take () } }
};
}
