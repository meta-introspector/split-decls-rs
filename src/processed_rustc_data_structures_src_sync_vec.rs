/* FP:vec.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_sync_vec_USE_0001
/* FP:vec.rs-0002 */ use std :: marker :: PhantomData ;
/* FP:vec.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_sync_vec_USE_0002
/* FP:vec.rs-0004 */ use crate :: rustc_index :: Idx ;
/* FP:vec.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_sync_vec_STRUCT_0003
/* FP:vec.rs-0006 */ # [derive (Default)] pub struct AppendOnlyIndexVec < I : Idx , T : Copy > { vec : elsa :: sync :: LockFreeFrozenVec < T > , _marker : PhantomData < fn (& I) > , }
/* FP:vec.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_sync_vec_IMPL_0004
/* FP:vec.rs-0008 */ impl < I : Idx , T : Copy > AppendOnlyIndexVec < I , T > { pub fn new () -> Self { Self { vec : elsa :: sync :: LockFreeFrozenVec :: new () , _marker : PhantomData } } pub fn push (& self , val : T) -> I { let i = self . vec . push (val) ; I :: new (i) } pub fn get (& self , i : I) -> Option < T > { let i = i . index () ; self . vec . get (i) } }
/* FP:vec.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_sync_vec_STRUCT_0005
/* FP:vec.rs-0010 */ # [derive (Default)] pub struct AppendOnlyVec < T : Copy > { vec : parking_lot :: RwLock < Vec < T > > , }
/* FP:vec.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_sync_vec_IMPL_0006
/* FP:vec.rs-0012 */ impl < T : Copy > AppendOnlyVec < T > { pub fn new () -> Self { Self { vec : Default :: default () } } pub fn push (& self , val : T) -> usize { let mut v = self . vec . write () ; let n = v . len () ; v . push (val) ; n } pub fn get (& self , i : usize) -> Option < T > { self . vec . read () . get (i) . copied () } pub fn iter_enumerated (& self) -> impl Iterator < Item = (usize , T) > { (0 ..) . map (| i | (i , self . get (i))) . take_while (| (_ , o) | o . is_some ()) . filter_map (| (i , o) | Some ((i , o ?))) } pub fn iter (& self) -> impl Iterator < Item = T > { (0 ..) . map (| i | self . get (i)) . take_while (| o | o . is_some ()) . flatten () } }
/* FP:vec.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_sync_vec_IMPL_0007
/* FP:vec.rs-0014 */ impl < T : Copy + PartialEq > AppendOnlyVec < T > { pub fn contains (& self , val : T) -> bool { self . iter_enumerated () . any (| (_ , v) | v == val) } }
/* FP:vec.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_sync_vec_IMPL_0008
/* FP:vec.rs-0016 */ impl < A : Copy > FromIterator < A > for AppendOnlyVec < A > { fn from_iter < T : IntoIterator < Item = A > > (iter : T) -> Self { let this = Self :: new () ; for val in iter { this . push (val) ; } this } }