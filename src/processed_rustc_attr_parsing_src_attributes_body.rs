/* FP:body.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_attr_parsing_src_attributes_body_USE_0001
/* FP:body.rs-0002 */ use super :: prelude :: * ;
/* FP:body.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_attr_parsing_src_attributes_body_STRUCT_0002
/* FP:body.rs-0004 */ pub (crate) struct CoroutineParser ;
/* FP:body.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_attr_parsing_src_attributes_body_IMPL_0003
/* FP:body.rs-0006 */ impl < S : Stage > NoArgsAttributeParser < S > for CoroutineParser { const PATH : & [Symbol] = & [sym :: coroutine] ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Error ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: Closure)]) ; const CREATE : fn (crate :: rustc_span :: Span) -> AttributeKind = | span | AttributeKind :: Coroutine (span) ; }