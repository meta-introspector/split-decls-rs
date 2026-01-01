/* FP:foreign_modules.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_metadata_src_foreign_modules_USE_0001
/* FP:foreign_modules.rs-0002 */ use crate :: rustc_data_structures :: fx :: FxIndexMap ;
/* FP:foreign_modules.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_metadata_src_foreign_modules_USE_0002
/* FP:foreign_modules.rs-0004 */ use rustc_hir as hir ;
/* FP:foreign_modules.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_metadata_src_foreign_modules_USE_0003
/* FP:foreign_modules.rs-0006 */ use crate :: rustc_complete :: def :: DefKind ;
/* FP:foreign_modules.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_metadata_src_foreign_modules_USE_0004
/* FP:foreign_modules.rs-0008 */ use crate :: rustc_complete :: def_id :: DefId ;
/* FP:foreign_modules.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_metadata_src_foreign_modules_USE_0005
/* FP:foreign_modules.rs-0010 */ use crate :: rustc_complete :: query :: LocalCrate ;
/* FP:foreign_modules.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_metadata_src_foreign_modules_USE_0006
/* FP:foreign_modules.rs-0012 */ use crate :: rustc_complete :: ty :: TyCtxt ;
/* FP:foreign_modules.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_metadata_src_foreign_modules_USE_0007
/* FP:foreign_modules.rs-0014 */ use crate :: rustc_complete :: cstore :: ForeignModule ;
/* FP:foreign_modules.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_metadata_src_foreign_modules_FN_0008
/* FP:foreign_modules.rs-0016 */ pub (crate) fn collect (tcx : TyCtxt < '_ > , LocalCrate : LocalCrate) -> FxIndexMap < DefId , ForeignModule > { let mut modules = FxIndexMap :: default () ; for id in tcx . hir_free_items () { if ! matches ! (tcx . def_kind (id . owner_id) , DefKind :: ForeignMod) { continue ; } let def_id = id . owner_id . to_def_id () ; let item = tcx . hir_item (id) ; if let hir :: ItemKind :: ForeignMod { abi , items } = item . kind { let foreign_items = items . iter () . map (| it | it . owner_id . to_def_id ()) . collect () ; modules . insert (def_id , ForeignModule { def_id , abi , foreign_items }) ; } } modules }