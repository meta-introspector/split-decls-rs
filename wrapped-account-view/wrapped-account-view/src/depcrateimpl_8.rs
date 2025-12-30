// Generated macro for impl_8 (impl)
macro_rules! Depcrateimpl_8 {
() => {
// Module: crate
// Provides: {"impl_8"}
// Dependencies: {}
impl < 'a , T : ? Sized > Ref < 'a , T > { # [doc = " Maps a reference to a new type."] # [inline] pub fn map < U : ? Sized , F > (orig : Ref < 'a , T > , f : F) -> Ref < 'a , U > where F : FnOnce (& T) -> & U , { let orig = ManuallyDrop :: new (orig) ; Ref { value : NonNull :: from (f (& * orig)) , state : orig . state , marker : PhantomData , } } # [doc = " Tries to makes a new `Ref` for a component of the borrowed data."] # [doc = ""] # [doc = " On failure, the original guard is returned alongside with the error"] # [doc = " returned by the closure."] # [inline] pub fn try_map < U : ? Sized , E > (orig : Ref < 'a , T > , f : impl FnOnce (& T) -> Result < & U , E > ,) -> Result < Ref < 'a , U > , (Self , E) > { let orig = ManuallyDrop :: new (orig) ; match f (& * orig) { Ok (value) => Ok (Ref { value : NonNull :: from (value) , state : orig . state , marker : PhantomData , }) , Err (e) => Err ((ManuallyDrop :: into_inner (orig) , e)) , } } # [doc = " Filters and maps a reference to a new type."] # [doc = ""] # [doc = " On failure, the original guard is returned."] # [inline] pub fn filter_map < U : ? Sized , F > (orig : Ref < 'a , T > , f : F) -> Result < Ref < 'a , U > , Self > where F : FnOnce (& T) -> Option < & U > , { let orig = ManuallyDrop :: new (orig) ; match f (& * orig) { Some (value) => Ok (Ref { value : NonNull :: from (value) , state : orig . state , marker : PhantomData , }) , None => Err (ManuallyDrop :: into_inner (orig)) , } } }
};
}
