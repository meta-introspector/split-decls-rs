/* FP:limits.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_interface_src_limits_USE_0001
/* FP:limits.rs-0002 */ use crate :: rustc_complete :: attrs :: AttributeKind ;
/* FP:limits.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_interface_src_limits_USE_0002
/* FP:limits.rs-0004 */ use crate :: rustc_complete :: limit :: Limit ;
/* FP:limits.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_interface_src_limits_USE_0003
/* FP:limits.rs-0006 */ use crate :: rustc_complete :: { Attribute , find_attr } ;
/* FP:limits.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_interface_src_limits_USE_0004
/* FP:limits.rs-0008 */ use crate :: rustc_complete :: query :: Providers ;
/* FP:limits.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_interface_src_limits_USE_0005
/* FP:limits.rs-0010 */ use crate :: rustc_complete :: Limits ;
/* FP:limits.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_interface_src_limits_FN_0006
/* FP:limits.rs-0012 */ pub (crate) fn provide (providers : & mut Providers) { providers . limits = | tcx , () | { let attrs = tcx . hir_krate_attrs () ; Limits { recursion_limit : get_recursion_limit (tcx . hir_krate_attrs ()) , move_size_limit : find_attr ! (attrs , AttributeKind :: MoveSizeLimit { limit , .. } => * limit) . unwrap_or (Limit :: new (tcx . sess . opts . unstable_opts . move_size_limit . unwrap_or (0))) , type_length_limit : find_attr ! (attrs , AttributeKind :: TypeLengthLimit { limit , .. } => * limit) . unwrap_or (Limit :: new (2usize . pow (24))) , pattern_complexity_limit : find_attr ! (attrs , AttributeKind :: PatternComplexityLimit { limit , .. } => * limit) . unwrap_or (Limit :: unlimited ()) , } } }
/* FP:limits.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_interface_src_limits_FN_0007
/* FP:limits.rs-0014 */ pub (crate) fn get_recursion_limit (attrs : & [Attribute]) -> Limit { find_attr ! (attrs , AttributeKind :: RecursionLimit { limit , .. } => * limit) . unwrap_or (Limit :: new (128)) }