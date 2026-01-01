/* FP:loop_match.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_attr_parsing_src_attributes_loop_match_USE_0001
/* FP:loop_match.rs-0002 */ use super :: prelude :: * ;
/* FP:loop_match.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_attr_parsing_src_attributes_loop_match_STRUCT_0002
/* FP:loop_match.rs-0004 */ pub (crate) struct LoopMatchParser ;
/* FP:loop_match.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_attr_parsing_src_attributes_loop_match_IMPL_0003
/* FP:loop_match.rs-0006 */ impl < S : Stage > NoArgsAttributeParser < S > for LoopMatchParser { const PATH : & [Symbol] = & [sym :: loop_match] ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Warn ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: Expression)]) ; const CREATE : fn (Span) -> AttributeKind = AttributeKind :: LoopMatch ; }
/* FP:loop_match.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_attr_parsing_src_attributes_loop_match_STRUCT_0004
/* FP:loop_match.rs-0008 */ pub (crate) struct ConstContinueParser ;
/* FP:loop_match.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_attr_parsing_src_attributes_loop_match_IMPL_0005
/* FP:loop_match.rs-0010 */ impl < S : Stage > NoArgsAttributeParser < S > for ConstContinueParser { const PATH : & [Symbol] = & [sym :: const_continue] ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Warn ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: Expression)]) ; const CREATE : fn (Span) -> AttributeKind = AttributeKind :: ConstContinue ; }