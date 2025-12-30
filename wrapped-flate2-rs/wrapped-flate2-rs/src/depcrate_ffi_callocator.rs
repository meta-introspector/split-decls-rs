// Generated macro for allocator (module)
macro_rules! Depcrate_ffi_callocator {
() => {
// Module: crate::ffi::c
// Provides: {"allocator"}
// Dependencies: {}
# [cfg (any (feature = "zlib-ng" , all (feature = "cloudflare_zlib" , not (feature = "zlib-rs") , not (feature = "zlib-ng")) , all (not (feature = "cloudflare_zlib") , not (feature = "zlib-ng") , not (feature = "zlib-rs")) ,))] mod allocator { use super :: * ; use std :: alloc :: { self , Layout } ; use std :: convert :: TryFrom ; use std :: os :: raw :: c_void ; const ALIGN : usize = std :: mem :: align_of :: < usize > () ; fn align_up (size : usize , align : usize) -> usize { (size + align - 1) & ! (align - 1) } pub extern "C" fn zalloc (_ptr : * mut c_void , items : uInt , item_size : uInt) -> * mut c_void { let size = match items . checked_mul (item_size) . and_then (| i | usize :: try_from (i) . ok ()) . map (| size | align_up (size , ALIGN)) . and_then (| i | i . checked_add (std :: mem :: size_of :: < usize > ())) { Some (i) => i , None => return ptr :: null_mut () , } ; let layout = match Layout :: from_size_align (size , ALIGN) { Ok (layout) => layout , Err (_) => return ptr :: null_mut () , } ; unsafe { let ptr = alloc :: alloc (layout) as * mut usize ; if ptr . is_null () { return ptr as * mut c_void ; } * ptr = size ; ptr . add (1) as * mut c_void } } pub extern "C" fn zfree (_ptr : * mut c_void , address : * mut c_void) { unsafe { let ptr = (address as * mut usize) . offset (- 1) ; let size = * ptr ; let layout = Layout :: from_size_align_unchecked (size , ALIGN) ; alloc :: dealloc (ptr as * mut u8 , layout) } } }
};
}
