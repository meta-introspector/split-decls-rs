// Generated macro for context (module)
macro_rules! Depcratecontext {
() => {
// Module: crate
// Provides: {"context"}
// Dependencies: {}
# [allow (unused)] mod context { use core :: cell :: UnsafeCell ; use core :: marker :: { PhantomData , PhantomPinned } ; use objc2 :: encode :: { Encoding , RefEncode } ; # [repr (C)] # [derive (Debug)] # [allow (missing_copy_implementations)] # [allow (unreachable_pub)] pub struct _CGLContextObject { inner : [u8 ; 0] , _p : UnsafeCell < PhantomData < (* const UnsafeCell < () > , PhantomPinned) > > , } unsafe impl RefEncode for _CGLContextObject { const ENCODING_REF : Encoding = Encoding :: Pointer (& Encoding :: Struct ("_CGLContextObject" , & [])) ; } }
};
}
