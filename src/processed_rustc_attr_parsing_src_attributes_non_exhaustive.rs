/* FP:non_exhaustive.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_attr_parsing_src_attributes_non_exhaustive_USE_0001
/* FP:non_exhaustive.rs-0002 */ use crate :: rustc_complete :: Target ;
/* FP:non_exhaustive.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_attr_parsing_src_attributes_non_exhaustive_USE_0002
/* FP:non_exhaustive.rs-0004 */ use crate :: rustc_complete :: attrs :: AttributeKind ;
/* FP:non_exhaustive.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_attr_parsing_src_attributes_non_exhaustive_USE_0003
/* FP:non_exhaustive.rs-0006 */ use crate :: rustc_complete :: { Span , Symbol , sym } ;
/* FP:non_exhaustive.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_attr_parsing_src_attributes_non_exhaustive_USE_0004
/* FP:non_exhaustive.rs-0008 */ use crate :: attributes :: { NoArgsAttributeParser , OnDuplicate } ;
/* FP:non_exhaustive.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_attr_parsing_src_attributes_non_exhaustive_USE_0005
/* FP:non_exhaustive.rs-0010 */ use crate :: context :: Stage ;
/* FP:non_exhaustive.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_attr_parsing_src_attributes_non_exhaustive_USE_0006
/* FP:non_exhaustive.rs-0012 */ use crate :: target_checking :: AllowedTargets ;
/* FP:non_exhaustive.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_attr_parsing_src_attributes_non_exhaustive_USE_0007
/* FP:non_exhaustive.rs-0014 */ use crate :: target_checking :: Policy :: { Allow , Warn } ;
/* FP:non_exhaustive.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_attr_parsing_src_attributes_non_exhaustive_STRUCT_0008
/* FP:non_exhaustive.rs-0016 */ pub (crate) struct NonExhaustiveParser ;
/* FP:non_exhaustive.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_attr_parsing_src_attributes_non_exhaustive_IMPL_0009
/* FP:non_exhaustive.rs-0018 */ impl < S : Stage > NoArgsAttributeParser < S > for NonExhaustiveParser { const PATH : & [Symbol] = & [sym :: non_exhaustive] ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Warn ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: Enum) , Allow (Target :: Struct) , Allow (Target :: Variant) , Warn (Target :: Field) , Warn (Target :: Arm) , Warn (Target :: MacroDef) , Warn (Target :: MacroCall) ,]) ; const CREATE : fn (Span) -> AttributeKind = AttributeKind :: NonExhaustive ; }