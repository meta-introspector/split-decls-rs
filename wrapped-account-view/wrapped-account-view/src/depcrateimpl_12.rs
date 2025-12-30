// Generated macro for impl_12 (impl)
macro_rules! Depcrateimpl_12 {
() => {
// Module: crate
// Provides: {"impl_12"}
// Dependencies: {}
impl < 'a , T : ? Sized > RefMut < 'a , T > { # [doc = " Maps a mutable reference to a new type."] # [inline] pub fn map < U : ? Sized , F > (orig : RefMut < 'a , T > , f : F) -> RefMut < 'a , U > where F : FnOnce (& mut T) -> & mut U , { let mut orig = ManuallyDrop :: new (orig) ; RefMut { value : NonNull :: from (f (& mut * orig)) , state : orig . state , marker : PhantomData , } } # [doc = " Tries to makes a new `RefMut` for a component of the borrowed data."] # [doc = ""] # [doc = " On failure, the original guard is returned alongside with the error"] # [doc = " returned by the closure."] # [inline] pub fn try_map < U : ? Sized , E > (orig : RefMut < 'a , T > , f : impl FnOnce (& mut T) -> Result < & mut U , E > ,) -> Result < RefMut < 'a , U > , (Self , E) > { let mut orig = ManuallyDrop :: new (orig) ; match f (& mut * orig) { Ok (value) => Ok (RefMut { value : NonNull :: from (value) , state : orig . state , marker : PhantomData , }) , Err (e) => Err ((ManuallyDrop :: into_inner (orig) , e)) , } } # [doc = " Filters and maps a mutable reference to a new type."] # [doc = ""] # [doc = " On failure, the original guard is returned alongside with the error"] # [doc = " returned by the closure."] # [inline] pub fn filter_map < U : ? Sized , F > (orig : RefMut < 'a , T > , f : F) -> Result < RefMut < 'a , U > , Self > where F : FnOnce (& mut T) -> Option < & mut U > , { let mut orig = ManuallyDrop :: new (orig) ; match f (& mut * orig) { Some (value) => Ok (RefMut { value : NonNull :: from (value) , state : orig . state , marker : PhantomData , }) , None => Err (ManuallyDrop :: into_inner (orig)) , } } }
};
}
