/* FP:flat_map_in_place.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_flat_map_in_place_USE_0001
/* FP:flat_map_in_place.rs-0002 */ use std :: { mem , ptr } ;
/* FP:flat_map_in_place.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_flat_map_in_place_USE_0002
/* FP:flat_map_in_place.rs-0004 */ use smallvec :: { Array , SmallVec } ;
/* FP:flat_map_in_place.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_flat_map_in_place_USE_0003
/* FP:flat_map_in_place.rs-0006 */ use thin_vec :: ThinVec ;
/* FP:flat_map_in_place.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_flat_map_in_place_TRAIT_0004
/* FP:flat_map_in_place.rs-0008 */ pub trait FlatMapInPlace < T > : Sized { fn flat_map_in_place < F , I > (& mut self , f : F) where F : FnMut (T) -> I , I : IntoIterator < Item = T > ; }
/* FP:flat_map_in_place.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_flat_map_in_place_MACRO_0005
/* FP:flat_map_in_place.rs-0010 */ macro_rules ! flat_map_in_place { ($ vec : ident $ (where T : $ bound : path) ?) => { fn flat_map_in_place < F , I > (& mut self , mut f : F) where F : FnMut (T) -> I , I : IntoIterator < Item = T >, { struct LeakGuard <'a , T $ (: $ bound) ?> (&'a mut $ vec < T >) ; impl <'a , T $ (: $ bound) ?> Drop for LeakGuard <'a , T > { fn drop (& mut self) { unsafe { self . 0 . set_len (0) ; } } } let this = LeakGuard (self) ; let mut read_i = 0 ; let mut write_i = 0 ; unsafe { while read_i < this . 0 . len () { let e = ptr :: read (this . 0 . as_ptr () . add (read_i)) ; let iter = f (e) . into_iter () ; read_i += 1 ; for e in iter { if write_i < read_i { ptr :: write (this . 0 . as_mut_ptr () . add (write_i) , e) ; write_i += 1 ; } else { this . 0 . insert (write_i , e) ; read_i += 1 ; write_i += 1 ; } } } this . 0 . set_len (write_i) ; mem :: forget (this) ; } } } ; }
/* FP:flat_map_in_place.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_flat_map_in_place_IMPL_0006
/* FP:flat_map_in_place.rs-0012 */ impl < T > FlatMapInPlace < T > for Vec < T > { flat_map_in_place ! (Vec) ; }
/* FP:flat_map_in_place.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_flat_map_in_place_IMPL_0007
/* FP:flat_map_in_place.rs-0014 */ impl < T , A : Array < Item = T > > FlatMapInPlace < T > for SmallVec < A > { flat_map_in_place ! (SmallVec where T : Array) ; }
/* FP:flat_map_in_place.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_flat_map_in_place_IMPL_0008
/* FP:flat_map_in_place.rs-0016 */ impl < T > FlatMapInPlace < T > for ThinVec < T > { flat_map_in_place ! (ThinVec) ; }