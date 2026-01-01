/* FP:mod.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_const_eval_src_util_mod_USE_0001
/* FP:mod.rs-0002 */ use crate :: rustc_complete :: mir ;
/* FP:mod.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_const_eval_src_util_mod_MOD_0002
/* FP:mod.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_const_eval_src_util_mod_MOD_0003
/* FP:mod.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_const_eval_src_util_mod_MOD_0004
/* FP:mod.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_const_eval_src_util_mod_MOD_0005
/* FP:mod.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_const_eval_src_util_mod_MOD_0006
/* FP:mod.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_const_eval_src_util_mod_USE_0007
/* FP:mod.rs-0014 */ pub use self :: alignment :: { is_disaligned , is_within_packed } ;
/* FP:mod.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_const_eval_src_util_mod_USE_0008
/* FP:mod.rs-0016 */ pub use self :: check_validity_requirement :: check_validity_requirement ;
/* FP:mod.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_const_eval_src_util_mod_USE_0009
/* FP:mod.rs-0018 */ pub (crate) use self :: check_validity_requirement :: validate_scalar_in_layout ;
/* FP:mod.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_const_eval_src_util_mod_USE_0010
/* FP:mod.rs-0020 */ pub use self :: compare_types :: { relate_types , sub_types } ;
/* FP:mod.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_const_eval_src_util_mod_USE_0011
/* FP:mod.rs-0022 */ pub use self :: type_name :: type_name ;
/* FP:mod.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_const_eval_src_util_mod_FN_0012
/* FP:mod.rs-0024 */ # [doc = " Classify whether an operator is \"left-homogeneous\", i.e., the LHS has the"] # [doc = " same type as the result."] # [inline] pub fn binop_left_homogeneous (op : mir :: BinOp) -> bool { use crate :: rustc_complete :: mir :: BinOp :: * ; match op { Add | AddUnchecked | Sub | SubUnchecked | Mul | MulUnchecked | Div | Rem | BitXor | BitAnd | BitOr | Offset | Shl | ShlUnchecked | Shr | ShrUnchecked => true , AddWithOverflow | SubWithOverflow | MulWithOverflow | Eq | Ne | Lt | Le | Gt | Ge | Cmp => { false } } }
/* FP:mod.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_const_eval_src_util_mod_FN_0013
/* FP:mod.rs-0026 */ # [doc = " Classify whether an operator is \"right-homogeneous\", i.e., the RHS has the"] # [doc = " same type as the LHS."] # [inline] pub fn binop_right_homogeneous (op : mir :: BinOp) -> bool { use crate :: rustc_complete :: mir :: BinOp :: * ; match op { Add | AddUnchecked | AddWithOverflow | Sub | SubUnchecked | SubWithOverflow | Mul | MulUnchecked | MulWithOverflow | Div | Rem | BitXor | BitAnd | BitOr | Eq | Ne | Lt | Le | Gt | Ge | Cmp => true , Offset | Shl | ShlUnchecked | Shr | ShrUnchecked => false , } }