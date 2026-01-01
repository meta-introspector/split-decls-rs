/* FP:idx.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_index_src_idx_USE_0001
/* FP:idx.rs-0002 */ use std :: fmt :: Debug ;
/* FP:idx.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_index_src_idx_USE_0002
/* FP:idx.rs-0004 */ use std :: hash :: Hash ;
/* FP:idx.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_index_src_idx_USE_0003
/* FP:idx.rs-0006 */ use std :: ops ;
/* FP:idx.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_index_src_idx_USE_0004
/* FP:idx.rs-0008 */ use std :: slice :: SliceIndex ;
/* FP:idx.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_index_src_idx_TRAIT_0005
/* FP:idx.rs-0010 */ # [doc = " Represents some newtyped `usize` wrapper."] # [doc = ""] # [doc = " Purpose: avoid mixing indexes for different bitvector domains."] pub trait Idx : Copy + 'static + Eq + PartialEq + Debug + Hash { fn new (idx : usize) -> Self ; fn index (self) -> usize ; # [inline] fn increment_by (& mut self , amount : usize) { * self = self . plus (amount) ; } # [inline] # [must_use = "Use `increment_by` if you wanted to update the index in-place"] fn plus (self , amount : usize) -> Self { Self :: new (self . index () + amount) } }
/* FP:idx.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_index_src_idx_IMPL_0006
/* FP:idx.rs-0012 */ impl Idx for usize { # [inline] fn new (idx : usize) -> Self { idx } # [inline] fn index (self) -> usize { self } }
/* FP:idx.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_index_src_idx_IMPL_0007
/* FP:idx.rs-0014 */ impl Idx for u32 { # [inline] fn new (idx : usize) -> Self { assert ! (idx <= u32 :: MAX as usize) ; idx as u32 } # [inline] fn index (self) -> usize { self as usize } }
/* FP:idx.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_index_src_idx_TRAIT_0008
/* FP:idx.rs-0016 */ # [doc = " Helper trait for indexing operations with a custom index type."] pub trait IntoSliceIdx < I , T : ? Sized > { type Output : SliceIndex < T > ; fn into_slice_idx (self) -> Self :: Output ; }
/* FP:idx.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_index_src_idx_IMPL_0009
/* FP:idx.rs-0018 */ impl < I : Idx , T > IntoSliceIdx < I , [T] > for I { type Output = usize ; # [inline] fn into_slice_idx (self) -> Self :: Output { self . index () } }
/* FP:idx.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_index_src_idx_IMPL_0010
/* FP:idx.rs-0020 */ impl < I , T > IntoSliceIdx < I , [T] > for ops :: RangeFull { type Output = ops :: RangeFull ; # [inline] fn into_slice_idx (self) -> Self :: Output { self } }
/* FP:idx.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_index_src_idx_IMPL_0011
/* FP:idx.rs-0022 */ impl < I : Idx , T > IntoSliceIdx < I , [T] > for ops :: Range < I > { type Output = ops :: Range < usize > ; # [inline] fn into_slice_idx (self) -> Self :: Output { ops :: Range { start : self . start . index () , end : self . end . index () } } }
/* FP:idx.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_index_src_idx_IMPL_0012
/* FP:idx.rs-0024 */ impl < I : Idx , T > IntoSliceIdx < I , [T] > for ops :: RangeFrom < I > { type Output = ops :: RangeFrom < usize > ; # [inline] fn into_slice_idx (self) -> Self :: Output { ops :: RangeFrom { start : self . start . index () } } }
/* FP:idx.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_index_src_idx_IMPL_0013
/* FP:idx.rs-0026 */ impl < I : Idx , T > IntoSliceIdx < I , [T] > for ops :: RangeTo < I > { type Output = ops :: RangeTo < usize > ; # [inline] fn into_slice_idx (self) -> Self :: Output { .. self . end . index () } }
/* FP:idx.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_index_src_idx_IMPL_0014
/* FP:idx.rs-0028 */ impl < I : Idx , T > IntoSliceIdx < I , [T] > for ops :: RangeInclusive < I > { type Output = ops :: RangeInclusive < usize > ; # [inline] fn into_slice_idx (self) -> Self :: Output { ops :: RangeInclusive :: new (self . start () . index () , self . end () . index ()) } }
/* FP:idx.rs-0029 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_index_src_idx_IMPL_0015
/* FP:idx.rs-0030 */ impl < I : Idx , T > IntoSliceIdx < I , [T] > for ops :: RangeToInclusive < I > { type Output = ops :: RangeToInclusive < usize > ; # [inline] fn into_slice_idx (self) -> Self :: Output { ..= self . end . index () } }
/* FP:idx.rs-0031 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_index_src_idx_IMPL_0016
/* FP:idx.rs-0032 */ # [cfg (feature = "nightly")] impl < I : Idx , T > IntoSliceIdx < I , [T] > for core :: range :: Range < I > { type Output = core :: range :: Range < usize > ; # [inline] fn into_slice_idx (self) -> Self :: Output { core :: range :: Range { start : self . start . index () , end : self . end . index () } } }
/* FP:idx.rs-0033 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_index_src_idx_IMPL_0017
/* FP:idx.rs-0034 */ # [cfg (feature = "nightly")] impl < I : Idx , T > IntoSliceIdx < I , [T] > for core :: range :: RangeFrom < I > { type Output = core :: range :: RangeFrom < usize > ; # [inline] fn into_slice_idx (self) -> Self :: Output { core :: range :: RangeFrom { start : self . start . index () } } }
/* FP:idx.rs-0035 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_index_src_idx_IMPL_0018
/* FP:idx.rs-0036 */ # [cfg (feature = "nightly")] impl < I : Idx , T > IntoSliceIdx < I , [T] > for core :: range :: RangeInclusive < I > { type Output = core :: range :: RangeInclusive < usize > ; # [inline] # [cfg (bootstrap)] fn into_slice_idx (self) -> Self :: Output { core :: range :: RangeInclusive { start : self . start . index () , end : self . end . index () } } # [inline] # [cfg (not (bootstrap))] fn into_slice_idx (self) -> Self :: Output { core :: range :: RangeInclusive { start : self . start . index () , last : self . last . index () } } }
/* FP:idx.rs-0037 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_index_src_idx_IMPL_0019
/* FP:idx.rs-0038 */ # [cfg (all (feature = "nightly" , not (bootstrap)))] impl < I : Idx , T > IntoSliceIdx < I , [T] > for core :: range :: RangeToInclusive < I > { type Output = core :: range :: RangeToInclusive < usize > ; # [inline] fn into_slice_idx (self) -> Self :: Output { core :: range :: RangeToInclusive { last : self . last . index () } } }