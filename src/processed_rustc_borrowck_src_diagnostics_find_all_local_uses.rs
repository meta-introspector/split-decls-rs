/* FP:find_all_local_uses.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_borrowck_src_diagnostics_find_all_local_uses_USE_0001
/* FP:find_all_local_uses.rs-0002 */ use std :: collections :: BTreeSet ;
/* FP:find_all_local_uses.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_borrowck_src_diagnostics_find_all_local_uses_USE_0002
/* FP:find_all_local_uses.rs-0004 */ use crate :: rustc_complete :: mir :: visit :: { PlaceContext , Visitor } ;
/* FP:find_all_local_uses.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_borrowck_src_diagnostics_find_all_local_uses_USE_0003
/* FP:find_all_local_uses.rs-0006 */ use crate :: rustc_complete :: mir :: { Body , Local , Location } ;
/* FP:find_all_local_uses.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_borrowck_src_diagnostics_find_all_local_uses_FN_0004
/* FP:find_all_local_uses.rs-0008 */ # [doc = " Find all uses of (including assignments to) a [`Local`]."] # [doc = ""] # [doc = " Uses `BTreeSet` so output is deterministic."] pub (super) fn find (body : & Body < '_ > , local : Local) -> BTreeSet < Location > { let mut visitor = AllLocalUsesVisitor { for_local : local , uses : BTreeSet :: default () } ; visitor . visit_body (body) ; visitor . uses }
/* FP:find_all_local_uses.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_borrowck_src_diagnostics_find_all_local_uses_STRUCT_0005
/* FP:find_all_local_uses.rs-0010 */ struct AllLocalUsesVisitor { for_local : Local , uses : BTreeSet < Location > , }
/* FP:find_all_local_uses.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_borrowck_src_diagnostics_find_all_local_uses_IMPL_0006
/* FP:find_all_local_uses.rs-0012 */ impl < 'tcx > Visitor < 'tcx > for AllLocalUsesVisitor { fn visit_local (& mut self , local : Local , _context : PlaceContext , location : Location) { if local == self . for_local { self . uses . insert (location) ; } } }