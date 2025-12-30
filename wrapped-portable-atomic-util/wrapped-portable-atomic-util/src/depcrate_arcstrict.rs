// Generated macro for strict (module)
macro_rules! Depcrate_arcstrict {
() => {
// Module: crate::arc
// Provides: {"strict"}
// Dependencies: {}
mod strict { # [cfg (portable_atomic_no_strict_provenance)] use core :: mem ; # [cfg (not (portable_atomic_no_strict_provenance))] # [allow (unused_imports)] pub (crate) use core :: ptr :: without_provenance_mut ; # [cfg (portable_atomic_no_strict_provenance)] # [inline (always)] # [must_use] pub (super) const fn without_provenance_mut < T > (addr : usize) -> * mut T { # [cfg (miri)] unsafe { mem :: transmute (addr) } # [cfg (not (miri))] { addr as * mut T } } # [doc = " Creates a new pointer with the metadata of `other`."] # [inline] # [must_use] pub (super) fn with_metadata_of < T , U : ? Sized > (this : * mut T , mut other : * mut U) -> * mut U { let target = & mut other as * mut * mut U as * mut * mut u8 ; unsafe { * target = this as * mut u8 } other } # [inline] # [must_use] pub (super) unsafe fn byte_add < T : ? Sized > (ptr : * mut T , count : usize) -> * mut T { unsafe { with_metadata_of ((ptr as * mut u8) . add (count) , ptr) } } # [inline] # [must_use] pub (super) unsafe fn byte_sub < T : ? Sized > (ptr : * mut T , count : usize) -> * mut T { unsafe { with_metadata_of ((ptr as * mut u8) . sub (count) , ptr) } } # [cfg (portable_atomic_no_strict_provenance)] pub (crate) trait PtrExt < T : ? Sized > : Copy { # [must_use] fn addr (self) -> usize ; } # [cfg (portable_atomic_no_strict_provenance)] impl < T : ? Sized > PtrExt < T > for * const T { # [inline (always)] # [must_use] fn addr (self) -> usize { unsafe { mem :: transmute (self as * const ()) } } } }
};
}
