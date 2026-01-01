/* FP:required_consts.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_required_consts_USE_0001
/* FP:required_consts.rs-0002 */ use crate :: rustc_complete :: mir :: visit :: Visitor ;
/* FP:required_consts.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_required_consts_USE_0002
/* FP:required_consts.rs-0004 */ use crate :: rustc_complete :: mir :: { Body , ConstOperand , Location , traversal } ;
/* FP:required_consts.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_required_consts_STRUCT_0003
/* FP:required_consts.rs-0006 */ pub (super) struct RequiredConstsVisitor < 'tcx > { required_consts : Vec < ConstOperand < 'tcx > > , }
/* FP:required_consts.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_required_consts_IMPL_0004
/* FP:required_consts.rs-0008 */ impl < 'tcx > RequiredConstsVisitor < 'tcx > { pub (super) fn compute_required_consts (body : & mut Body < 'tcx >) { let mut visitor = RequiredConstsVisitor { required_consts : Vec :: new () } ; for (bb , bb_data) in traversal :: reverse_postorder (& body) { visitor . visit_basic_block_data (bb , bb_data) ; } body . set_required_consts (visitor . required_consts) ; } }
/* FP:required_consts.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_mir_transform_src_required_consts_IMPL_0005
/* FP:required_consts.rs-0010 */ impl < 'tcx > Visitor < 'tcx > for RequiredConstsVisitor < 'tcx > { fn visit_const_operand (& mut self , constant : & ConstOperand < 'tcx > , _ : Location) { if constant . const_ . is_required_const () { self . required_consts . push (* constant) ; } } }