// Generated macro for impl_269 (impl)
macro_rules! Depcrate_pathimpl_269 {
() => {
// Module: crate::path
// Provides: {"impl_269"}
// Dependencies: {}
impl CGPath { pub fn from_rect (rect : CGRect , transform : Option < & CGAffineTransform >) -> CGPath { unsafe { let transform = match transform { None => ptr :: null () , Some (transform) => transform as * const CGAffineTransform , } ; CGPath :: from_ptr (CGPathCreateWithRect (rect , transform)) } } pub fn type_id () -> CFTypeID { unsafe { CGPathGetTypeID () } } pub fn apply < 'a , F > (& 'a self , mut closure : & 'a F) where F : FnMut (CGPathElementRef < 'a >) , { unsafe { CGPathApply (self . as_ptr () , & mut closure as * mut _ as * mut c_void , do_apply :: < F > ,) ; } unsafe extern "C" fn do_apply < 'a , F > (info : * mut c_void , element : * const CGPathElement) where F : FnMut (CGPathElementRef < 'a >) , { let closure = info as * mut * mut F ; (* * closure) (CGPathElementRef :: new (element)) } } }
};
}
