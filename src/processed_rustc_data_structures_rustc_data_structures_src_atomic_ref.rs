/* FP:atomic_ref.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_atomic_ref_USE_0001
/* FP:atomic_ref.rs-0002 */ use std :: marker :: PhantomData ;
/* FP:atomic_ref.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_atomic_ref_USE_0002
/* FP:atomic_ref.rs-0004 */ use std :: sync :: atomic :: { AtomicPtr , Ordering } ;
/* FP:atomic_ref.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_atomic_ref_STRUCT_0003
/* FP:atomic_ref.rs-0006 */ # [doc = " This is essentially an `AtomicPtr` but is guaranteed to always be valid"] pub struct AtomicRef < T : 'static > (AtomicPtr < T > , PhantomData < & 'static T >) ;
/* FP:atomic_ref.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_atomic_ref_IMPL_0004
/* FP:atomic_ref.rs-0008 */ impl < T : 'static > AtomicRef < T > { pub const fn new (initial : & 'static T) -> AtomicRef < T > { AtomicRef (AtomicPtr :: new (initial as * const T as * mut T) , PhantomData) } pub fn swap (& self , new : & 'static T) -> & 'static T { unsafe { & * self . 0 . swap (new as * const T as * mut T , Ordering :: SeqCst) } } }
/* FP:atomic_ref.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_atomic_ref_IMPL_0005
/* FP:atomic_ref.rs-0010 */ impl < T : 'static > std :: ops :: Deref for AtomicRef < T > { type Target = T ; fn deref (& self) -> & Self :: Target { unsafe { & * self . 0 . load (Ordering :: SeqCst) } } }