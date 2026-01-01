/* FP:dummy.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_attr_parsing_src_attributes_dummy_USE_0001
/* FP:dummy.rs-0002 */ use crate :: rustc_feature :: { AttributeTemplate , template } ;
/* FP:dummy.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_attr_parsing_src_attributes_dummy_USE_0002
/* FP:dummy.rs-0004 */ use crate :: rustc_complete :: attrs :: AttributeKind ;
/* FP:dummy.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_attr_parsing_src_attributes_dummy_USE_0003
/* FP:dummy.rs-0006 */ use crate :: rustc_complete :: { Symbol , sym } ;
/* FP:dummy.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_attr_parsing_src_attributes_dummy_USE_0004
/* FP:dummy.rs-0008 */ use crate :: attributes :: { AttributeOrder , OnDuplicate , SingleAttributeParser } ;
/* FP:dummy.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_attr_parsing_src_attributes_dummy_USE_0005
/* FP:dummy.rs-0010 */ use crate :: context :: { AcceptContext , Stage } ;
/* FP:dummy.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_attr_parsing_src_attributes_dummy_USE_0006
/* FP:dummy.rs-0012 */ use crate :: parser :: ArgParser ;
/* FP:dummy.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_attr_parsing_src_attributes_dummy_USE_0007
/* FP:dummy.rs-0014 */ use crate :: target_checking :: { ALL_TARGETS , AllowedTargets } ;
/* FP:dummy.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_attr_parsing_src_attributes_dummy_STRUCT_0008
/* FP:dummy.rs-0016 */ pub (crate) struct DummyParser ;
/* FP:dummy.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_attr_parsing_src_attributes_dummy_IMPL_0009
/* FP:dummy.rs-0018 */ impl < S : Stage > SingleAttributeParser < S > for DummyParser { const PATH : & [Symbol] = & [sym :: rustc_dummy] ; const ATTRIBUTE_ORDER : AttributeOrder = AttributeOrder :: KeepInnermost ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Ignore ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (ALL_TARGETS) ; const TEMPLATE : AttributeTemplate = template ! (Word) ; fn convert (_ : & mut AcceptContext < '_ , '_ , S > , _ : & ArgParser < '_ >) -> Option < AttributeKind > { Some (AttributeKind :: Dummy) } }