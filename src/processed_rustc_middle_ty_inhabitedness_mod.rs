/* FP:mod.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_inhabitedness_mod_USE_0001
/* FP:mod.rs-0002 */ use rustc_type_ir :: TyKind :: * ;
/* FP:mod.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_inhabitedness_mod_USE_0002
/* FP:mod.rs-0004 */ use tracing :: instrument ;
/* FP:mod.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_inhabitedness_mod_USE_0003
/* FP:mod.rs-0006 */ use crate :: query :: Providers ;
/* FP:mod.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_inhabitedness_mod_USE_0004
/* FP:mod.rs-0008 */ use crate :: ty :: context :: TyCtxt ;
/* FP:mod.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_inhabitedness_mod_USE_0005
/* FP:mod.rs-0010 */ use crate :: ty :: { self , DefId , Ty , TypeVisitableExt , VariantDef , Visibility } ;
/* FP:mod.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_inhabitedness_mod_MOD_0006
/* FP:mod.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_inhabitedness_mod_USE_0007
/* FP:mod.rs-0014 */ pub use inhabited_predicate :: InhabitedPredicate ;
/* FP:mod.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_inhabitedness_mod_FN_0008
/* FP:mod.rs-0016 */ pub (crate) fn provide (providers : & mut Providers) { * providers = Providers { inhabited_predicate_adt , inhabited_predicate_type , .. * providers } ; }
/* FP:mod.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_inhabitedness_mod_FN_0009
/* FP:mod.rs-0018 */ # [doc = " Returns an `InhabitedPredicate` that is generic over type parameters and"] # [doc = " requires calling [`InhabitedPredicate::instantiate`]"] fn inhabited_predicate_adt (tcx : TyCtxt < '_ > , def_id : DefId) -> InhabitedPredicate < '_ > { if let Some (def_id) = def_id . as_local () { if matches ! (tcx . representability (def_id) , ty :: Representability :: Infinite (_)) { return InhabitedPredicate :: True ; } } let adt = tcx . adt_def (def_id) ; InhabitedPredicate :: any (tcx , adt . variants () . iter () . map (| variant | variant . inhabited_predicate (tcx , adt)) ,) }
/* FP:mod.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_inhabitedness_mod_IMPL_0010
/* FP:mod.rs-0020 */ impl < 'tcx > VariantDef { # [doc = " Calculates the forest of `DefId`s from which this variant is visibly uninhabited."] pub fn inhabited_predicate (& self , tcx : TyCtxt < 'tcx > , adt : ty :: AdtDef < '_ > ,) -> InhabitedPredicate < 'tcx > { debug_assert ! (! adt . is_union ()) ; InhabitedPredicate :: all (tcx , self . fields . iter () . map (| field | { let pred = tcx . type_of (field . did) . instantiate_identity () . inhabited_predicate (tcx) ; if adt . is_enum () { return pred ; } match field . vis { Visibility :: Public => pred , Visibility :: Restricted (from) => { pred . or (tcx , InhabitedPredicate :: NotInModule (from)) } } }) ,) } }
/* FP:mod.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_inhabitedness_mod_IMPL_0011
/* FP:mod.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_inhabitedness_mod_FN_0012
/* FP:mod.rs-0024 */ # [doc = " N.B. this query should only be called through `Ty::inhabited_predicate`"] fn inhabited_predicate_type < 'tcx > (tcx : TyCtxt < 'tcx > , ty : Ty < 'tcx >) -> InhabitedPredicate < 'tcx > { match * ty . kind () { Adt (adt , args) => tcx . inhabited_predicate_adt (adt . did ()) . instantiate (tcx , args) , Tuple (tys) => { InhabitedPredicate :: all (tcx , tys . iter () . map (| ty | ty . inhabited_predicate (tcx))) } Array (ty , len) => match len . try_to_target_usize (tcx) { Some (0) => InhabitedPredicate :: True , Some (1 ..) => ty . inhabited_predicate (tcx) , None => ty . inhabited_predicate (tcx) . or (tcx , InhabitedPredicate :: ConstIsZero (len)) , } , _ => bug ! ("unexpected TyKind, use `Ty::inhabited_predicate`") , } }