/* FP:place_ext.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_borrowck_src_place_ext_USE_0001
/* FP:place_ext.rs-0002 */ use rustc_hir as hir ;
/* FP:place_ext.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_borrowck_src_place_ext_USE_0002
/* FP:place_ext.rs-0004 */ use rustc_macros :: extension ;
/* FP:place_ext.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_borrowck_src_place_ext_USE_0003
/* FP:place_ext.rs-0006 */ use crate :: rustc_complete :: mir :: { Body , Mutability , Place , ProjectionElem } ;
/* FP:place_ext.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_borrowck_src_place_ext_USE_0004
/* FP:place_ext.rs-0008 */ use crate :: rustc_complete :: ty :: { self , TyCtxt } ;
/* FP:place_ext.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_borrowck_src_place_ext_USE_0005
/* FP:place_ext.rs-0010 */ use tracing :: debug ;
/* FP:place_ext.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_borrowck_src_place_ext_USE_0006
/* FP:place_ext.rs-0012 */ use crate :: borrow_set :: LocalsStateAtExit ;
/* FP:place_ext.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_borrowck_src_place_ext_IMPL_0007
/* FP:place_ext.rs-0014 */ # [extension (pub trait PlaceExt <'tcx >)] impl < 'tcx > Place < 'tcx > { # [doc = " Returns `true` if we can safely ignore borrows of this place."] # [doc = " This is true whenever there is no action that the user can do"] # [doc = " to the place `self` that would invalidate the borrow. This is true"] # [doc = " for borrows of raw pointer dereferents as well as shared references."] fn ignore_borrow (& self , tcx : TyCtxt < 'tcx > , body : & Body < 'tcx > , locals_state_at_exit : & LocalsStateAtExit ,) -> bool { if let LocalsStateAtExit :: SomeAreInvalidated { has_storage_dead_or_moved } = locals_state_at_exit { let ignore = ! has_storage_dead_or_moved . contains (self . local) && body . local_decls [self . local] . mutability == Mutability :: Not ; debug ! ("ignore_borrow: local {:?} => {:?}" , self . local , ignore) ; if ignore { return true ; } } for (i , (proj_base , elem)) in self . iter_projections () . enumerate () { if elem == ProjectionElem :: Deref { let ty = proj_base . ty (body , tcx) . ty ; match ty . kind () { ty :: Ref (_ , _ , hir :: Mutability :: Not) if i == 0 => { if body . local_decls [self . local] . is_ref_to_thread_local () { continue ; } return true ; } ty :: RawPtr (..) | ty :: Ref (_ , _ , hir :: Mutability :: Not) => { return true ; } _ => { } } } } false } }