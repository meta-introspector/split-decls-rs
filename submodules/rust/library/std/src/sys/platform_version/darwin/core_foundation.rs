mkuse!{use super :: root_relative ;}
mkuse!{use crate :: ffi :: { CStr , c_char , c_void } ;}
mkuse!{use crate :: ptr :: null_mut ;}
mkuse!{use crate :: sys :: common :: small_c_string :: run_path_with_cstr ;}
mkitem!{pub (super) type Boolean = u8 ;}
mkitem!{pub (super) type CFTypeID = usize ;}
mkitem!{pub (super) type CFOptionFlags = usize ;}
mkitem!{pub (super) type CFIndex = isize ;}
mkitem!{pub (super) type CFTypeRef = * mut c_void ;}
mkitem!{pub (super) type CFAllocatorRef = CFTypeRef ;}
mkitem!{pub (super) const kCFAllocatorDefault : CFAllocatorRef = null_mut () ;}
mkitem!{pub (super) type CFErrorRef = CFTypeRef ;}
mkitem!{pub (super) type CFDataRef = CFTypeRef ;}
mkitem!{pub (super) const kCFPropertyListImmutable : CFOptionFlags = 0 ;}
mkitem!{pub (super) type CFPropertyListFormat = CFIndex ;}
mkitem!{pub (super) type CFPropertyListRef = CFTypeRef ;}
mkitem!{pub (super) type CFStringRef = CFTypeRef ;}
mkitem!{pub (super) type CFStringEncoding = u32 ;}
mkitem!{pub (super) const kCFStringEncodingUTF8 : CFStringEncoding = 0x08000100 ;}
mkitem!{pub (super) type CFDictionaryRef = CFTypeRef ;}
mkitem!{mkstruct!{# [doc = " An open handle to the dynamically loaded CoreFoundation framework."] # [doc = ""] # [doc = " This is `dlopen`ed, and later `dlclose`d. This is done to try to avoid"] # [doc = " \"leaking\" the CoreFoundation symbols to the rest of the user's binary if"] # [doc = " they decided to not link CoreFoundation themselves."] # [doc = ""] # [doc = " It is also faster to look up symbols directly via this handle than with"] # [doc = " `RTLD_DEFAULT`."] pub (super) struct CFHandle (* mut c_void) ;}}
mkitem!{macro_rules ! dlsym_fn { (unsafe fn $ name : ident ($ ($ param : ident : $ param_ty : ty) ,* $ (,) ?) $ (-> $ ret : ty) ?;) => { pub (super) unsafe fn $ name (& self , $ ($ param : $ param_ty) ,*) $ (-> $ ret) ? { let ptr = unsafe { libc :: dlsym (self . 0 , concat ! (stringify ! ($ name) , '\0') . as_bytes () . as_ptr () . cast () ,) } ; if ptr . is_null () { let err = unsafe { CStr :: from_ptr (libc :: dlerror ()) } ; panic ! ("could not find function {}: {err:?}" , stringify ! ($ name)) ; } let fnptr = unsafe { crate :: mem :: transmute ::< * mut c_void , unsafe extern "C" fn ($ ($ param_ty) ,*) $ (-> $ ret) ?, > (ptr) } ; unsafe { fnptr ($ ($ param) ,*) } } } ; }}
mkitem!{mkimpl!{impl CFHandle { # [doc = " Link to the CoreFoundation dylib, and look up symbols from that."] pub (super) fn new () -> Self { let cf_path = root_relative ("/System/Library/Frameworks/CoreFoundation.framework/CoreFoundation") ; let handle = run_path_with_cstr (& cf_path , & | path | unsafe { Ok (libc :: dlopen (path . as_ptr () , libc :: RTLD_LAZY | libc :: RTLD_LOCAL)) }) . expect ("failed allocating string") ; if handle . is_null () { let err = unsafe { CStr :: from_ptr (libc :: dlerror ()) } ; panic ! ("could not open CoreFoundation.framework: {err:?}") ; } Self (handle) } pub (super) fn kCFAllocatorNull (& self) -> CFAllocatorRef { let static_ptr = unsafe { libc :: dlsym (self . 0 , c"kCFAllocatorNull" . as_ptr ()) } ; if static_ptr . is_null () { let err = unsafe { CStr :: from_ptr (libc :: dlerror ()) } ; panic ! ("could not find kCFAllocatorNull: {err:?}") ; } unsafe { * static_ptr . cast () } } dlsym_fn ! (unsafe fn CFRelease (cf : CFTypeRef) ;) ; dlsym_fn ! (unsafe fn CFGetTypeID (cf : CFTypeRef) -> CFTypeID ;) ; dlsym_fn ! (unsafe fn CFDataCreateWithBytesNoCopy (allocator : CFAllocatorRef , bytes : * const u8 , length : CFIndex , bytes_deallocator : CFAllocatorRef ,) -> CFDataRef ;) ; dlsym_fn ! (unsafe fn CFPropertyListCreateWithData (allocator : CFAllocatorRef , data : CFDataRef , options : CFOptionFlags , format : * mut CFPropertyListFormat , error : * mut CFErrorRef ,) -> CFPropertyListRef ;) ; dlsym_fn ! (unsafe fn CFStringGetTypeID () -> CFTypeID ;) ; dlsym_fn ! (unsafe fn CFStringCreateWithCStringNoCopy (alloc : CFAllocatorRef , c_str : * const c_char , encoding : CFStringEncoding , contents_deallocator : CFAllocatorRef ,) -> CFStringRef ;) ; dlsym_fn ! (unsafe fn CFStringGetCString (the_string : CFStringRef , buffer : * mut c_char , buffer_size : CFIndex , encoding : CFStringEncoding ,) -> Boolean ;) ; dlsym_fn ! (unsafe fn CFDictionaryGetTypeID () -> CFTypeID ;) ; dlsym_fn ! (unsafe fn CFDictionaryGetValue (the_dict : CFDictionaryRef , key : * const c_void ,) -> * const c_void ;) ; }}}
mkitem!{mkimpl!{impl Drop for CFHandle { fn drop (& mut self) { let _ = unsafe { libc :: dlclose (self . 0) } ; } }}}