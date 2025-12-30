// Generated macro for impl_867 (impl)
macro_rules! Depcrate_rc_weakimpl_867 {
() => {
// Module: crate::rc::weak
// Provides: {"impl_867"}
// Dependencies: {}
impl < T : Message > Weak < T > { # [doc = " Construct a new weak pointer that references the given object."] # [doc (alias = "objc_initWeak")] # [inline] pub fn new (obj : & T) -> Self { unsafe { Self :: new_inner (obj) } } # [doc = " Construct a new weak pointer that references the given [`Retained`]."] # [doc (alias = "objc_initWeak")] # [deprecated = "use `Weak::from_retained` instead"] # [inline] pub fn from_id (obj : & Retained < T >) -> Self { Self :: from_retained (obj) } # [doc = " Construct a new weak pointer that references the given [`Retained`]."] # [doc (alias = "objc_initWeak")] # [inline] pub fn from_retained (obj : & Retained < T >) -> Self { unsafe { Self :: new_inner (Retained :: as_ptr (obj)) } } # [doc = " Raw constructor."] # [doc = ""] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The object must be valid or null."] unsafe fn new_inner (obj : * const T) -> Self { let inner = Box :: new (UnsafeCell :: new (ptr :: null_mut ())) ; let _ = unsafe { ffi :: objc_initWeak (inner . get () , (obj as * mut T) . cast ()) } ; Self { inner , item : PhantomData , } } # [doc = " Load the object into an [`Retained`] if it still exists."] # [doc = ""] # [doc = " Returns [`None`] if the object has been deallocated, or the `Weak`"] # [doc = " was created with [`Default::default`]."] # [doc (alias = "retain")] # [doc (alias = "objc_loadWeak")] # [doc (alias = "objc_loadWeakRetained")] # [inline] pub fn load (& self) -> Option < Retained < T > > { let ptr = self . inner . get () ; let obj = unsafe { ffi :: objc_loadWeakRetained (ptr) } . cast () ; unsafe { Retained :: from_raw (obj) } } }
};
}
