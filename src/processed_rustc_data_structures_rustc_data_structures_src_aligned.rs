/* FP:aligned.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_aligned_USE_0001
/* FP:aligned.rs-0002 */ use std :: marker :: PointeeSized ;
/* FP:aligned.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_aligned_USE_0002
/* FP:aligned.rs-0004 */ use std :: ptr :: Alignment ;
/* FP:aligned.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_aligned_FN_0003
/* FP:aligned.rs-0006 */ # [doc = " Returns the ABI-required minimum alignment of a type in bytes."] # [doc = ""] # [doc = " This is equivalent to [`align_of`], but also works for some unsized"] # [doc = " types (e.g. slices or rustc's `List`s)."] pub const fn align_of < T : ? Sized + Aligned > () -> Alignment { T :: ALIGN }
/* FP:aligned.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_aligned_TRAIT_0004
/* FP:aligned.rs-0008 */ # [doc = " A type with a statically known alignment."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `Self::ALIGN` must be equal to the alignment of `Self`. For sized types it"] # [doc = " is [`align_of::<Self>()`], for unsized types it depends on the type, for"] # [doc = " example `[T]` has alignment of `T`."] # [doc = ""] # [doc = " [`align_of::<Self>()`]: align_of"] pub unsafe trait Aligned : PointeeSized { # [doc = " Alignment of `Self`."] const ALIGN : Alignment ; }
/* FP:aligned.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_aligned_IMPL_0005
/* FP:aligned.rs-0010 */ unsafe impl < T > Aligned for T { const ALIGN : Alignment = Alignment :: of :: < Self > () ; }
/* FP:aligned.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_aligned_IMPL_0006
/* FP:aligned.rs-0012 */ unsafe impl < T > Aligned for [T] { const ALIGN : Alignment = Alignment :: of :: < T > () ; }