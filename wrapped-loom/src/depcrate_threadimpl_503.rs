// Generated macro for impl_503 (impl)
macro_rules! Depcrate_threadimpl_503 {
() => {
// Module: crate::thread
// Provides: {"impl_503"}
// Dependencies: {}
impl < T : 'static > LocalKey < T > { # [doc = " Mock implementation of `std::thread::LocalKey::with`."] pub fn with < F , R > (& 'static self , f : F) -> R where F : FnOnce (& T) -> R , { self . try_with (f) . expect ("cannot access a (mock) TLS value during or after it is destroyed") } # [doc = " Mock implementation of `std::thread::LocalKey::try_with`."] pub fn try_with < F , R > (& 'static self , f : F) -> Result < R , AccessError > where F : FnOnce (& T) -> R , { let value = match unsafe { self . get () } { Some (v) => v ? , None => { let value = (self . init) () ; rt :: execution (| execution | { trace ! ("LocalKey::try_with") ; execution . threads . local_init (self , value) ; }) ; unsafe { self . get () } . expect ("bug") ? } } ; Ok (f (value)) } unsafe fn get (& 'static self) -> Option < Result < & 'static T , AccessError > > { unsafe fn transmute_lt < 'a , 'b , T > (t : & 'a T) -> & 'b T { std :: mem :: transmute :: < & 'a T , & 'b T > (t) } rt :: execution (| execution | { trace ! ("LocalKey::get") ; let res = execution . threads . local (self) ? ; let local = match res { Ok (l) => l , Err (e) => return Some (Err (e)) , } ; Some (Ok (transmute_lt (local))) }) } }
};
}
