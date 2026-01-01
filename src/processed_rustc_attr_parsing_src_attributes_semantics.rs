/* FP:semantics.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_attr_parsing_src_attributes_semantics_USE_0001
/* FP:semantics.rs-0002 */ use super :: prelude :: * ;
/* FP:semantics.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_attr_parsing_src_attributes_semantics_STRUCT_0002
/* FP:semantics.rs-0004 */ pub (crate) struct MayDangleParser ;
/* FP:semantics.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_attr_parsing_src_attributes_semantics_IMPL_0003
/* FP:semantics.rs-0006 */ impl < S : Stage > NoArgsAttributeParser < S > for MayDangleParser { const PATH : & [Symbol] = & [sym :: may_dangle] ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Warn ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (ALL_TARGETS) ; const CREATE : fn (span : Span) -> AttributeKind = AttributeKind :: MayDangle ; }