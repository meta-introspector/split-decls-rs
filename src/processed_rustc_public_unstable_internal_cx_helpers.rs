/* FP:helpers.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_unstable_internal_cx_helpers_USE_0001
/* FP:helpers.rs-0002 */ use crate :: rustc_complete :: ty ;
/* FP:helpers.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_unstable_internal_cx_helpers_TRAIT_0002
/* FP:helpers.rs-0004 */ pub (crate) trait ExistentialProjectionHelpers < 'tcx > { fn new_from_args (& self , def_id : crate :: rustc_span :: def_id :: DefId , args : ty :: GenericArgsRef < 'tcx > , term : ty :: Term < 'tcx > ,) -> ty :: ExistentialProjection < 'tcx > ; }
/* FP:helpers.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_unstable_internal_cx_helpers_TRAIT_0003
/* FP:helpers.rs-0006 */ pub (crate) trait ExistentialTraitRefHelpers < 'tcx > { fn new_from_args (& self , trait_def_id : crate :: rustc_span :: def_id :: DefId , args : ty :: GenericArgsRef < 'tcx > ,) -> ty :: ExistentialTraitRef < 'tcx > ; }
/* FP:helpers.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_public_src_unstable_internal_cx_helpers_TRAIT_0004
/* FP:helpers.rs-0008 */ pub (crate) trait TraitRefHelpers < 'tcx > { fn new_from_args (& self , trait_def_id : crate :: rustc_span :: def_id :: DefId , args : ty :: GenericArgsRef < 'tcx > ,) -> ty :: TraitRef < 'tcx > ; }