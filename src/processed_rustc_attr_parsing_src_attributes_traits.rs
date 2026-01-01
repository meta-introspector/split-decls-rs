/* FP:traits.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_attr_parsing_src_attributes_traits_USE_0001
/* FP:traits.rs-0002 */ use std :: mem ;
/* FP:traits.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_attr_parsing_src_attributes_traits_USE_0002
/* FP:traits.rs-0004 */ use crate :: rustc_feature :: AttributeType ;
/* FP:traits.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_attr_parsing_src_attributes_traits_USE_0003
/* FP:traits.rs-0006 */ use super :: prelude :: * ;
/* FP:traits.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_attr_parsing_src_attributes_traits_USE_0004
/* FP:traits.rs-0008 */ use crate :: attributes :: { AttributeOrder , NoArgsAttributeParser , OnDuplicate , SingleAttributeParser , } ;
/* FP:traits.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_attr_parsing_src_attributes_traits_USE_0005
/* FP:traits.rs-0010 */ use crate :: context :: { AcceptContext , Stage } ;
/* FP:traits.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_attr_parsing_src_attributes_traits_USE_0006
/* FP:traits.rs-0012 */ use crate :: parser :: ArgParser ;
/* FP:traits.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_attr_parsing_src_attributes_traits_USE_0007
/* FP:traits.rs-0014 */ use crate :: target_checking :: Policy :: { Allow , Warn } ;
/* FP:traits.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_attr_parsing_src_attributes_traits_USE_0008
/* FP:traits.rs-0016 */ use crate :: target_checking :: { ALL_TARGETS , AllowedTargets } ;
/* FP:traits.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_attr_parsing_src_attributes_traits_STRUCT_0009
/* FP:traits.rs-0018 */ pub (crate) struct SkipDuringMethodDispatchParser ;
/* FP:traits.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_attr_parsing_src_attributes_traits_IMPL_0010
/* FP:traits.rs-0020 */ impl < S : Stage > SingleAttributeParser < S > for SkipDuringMethodDispatchParser { const PATH : & [Symbol] = & [sym :: rustc_skip_during_method_dispatch] ; const ATTRIBUTE_ORDER : AttributeOrder = AttributeOrder :: KeepInnermost ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Error ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: Trait)]) ; const TEMPLATE : AttributeTemplate = template ! (List : & ["array, boxed_slice"]) ; fn convert (cx : & mut AcceptContext < '_ , '_ , S > , args : & ArgParser < '_ >) -> Option < AttributeKind > { let mut array = false ; let mut boxed_slice = false ; let Some (args) = args . list () else { cx . expected_list (cx . attr_span) ; return None ; } ; if args . is_empty () { cx . expected_at_least_one_argument (args . span) ; return None ; } for arg in args . mixed () { let Some (arg) = arg . meta_item () else { cx . unexpected_literal (arg . span ()) ; continue ; } ; if let Err (span) = arg . args () . no_args () { cx . expected_no_args (span) ; } let path = arg . path () ; let (key , skip) : (Symbol , & mut bool) = match path . word_sym () { Some (key @ sym :: array) => (key , & mut array) , Some (key @ sym :: boxed_slice) => (key , & mut boxed_slice) , _ => { cx . expected_specific_argument (path . span () , & [sym :: array , sym :: boxed_slice]) ; continue ; } } ; if mem :: replace (skip , true) { cx . duplicate_key (arg . span () , key) ; } } Some (AttributeKind :: SkipDuringMethodDispatch { array , boxed_slice , span : cx . attr_span }) } }
/* FP:traits.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_attr_parsing_src_attributes_traits_STRUCT_0011
/* FP:traits.rs-0022 */ pub (crate) struct ParenSugarParser ;
/* FP:traits.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_attr_parsing_src_attributes_traits_IMPL_0012
/* FP:traits.rs-0024 */ impl < S : Stage > NoArgsAttributeParser < S > for ParenSugarParser { const PATH : & [Symbol] = & [sym :: rustc_paren_sugar] ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Error ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: Trait)]) ; const CREATE : fn (Span) -> AttributeKind = AttributeKind :: ParenSugar ; }
/* FP:traits.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_attr_parsing_src_attributes_traits_STRUCT_0013
/* FP:traits.rs-0026 */ pub (crate) struct TypeConstParser ;
/* FP:traits.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_attr_parsing_src_attributes_traits_IMPL_0014
/* FP:traits.rs-0028 */ impl < S : Stage > NoArgsAttributeParser < S > for TypeConstParser { const PATH : & [Symbol] = & [sym :: type_const] ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Error ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: AssocConst)]) ; const CREATE : fn (Span) -> AttributeKind = AttributeKind :: TypeConst ; }
/* FP:traits.rs-0029 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_attr_parsing_src_attributes_traits_STRUCT_0015
/* FP:traits.rs-0030 */ pub (crate) struct MarkerParser ;
/* FP:traits.rs-0031 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_attr_parsing_src_attributes_traits_IMPL_0016
/* FP:traits.rs-0032 */ impl < S : Stage > NoArgsAttributeParser < S > for MarkerParser { const PATH : & [Symbol] = & [sym :: marker] ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Warn ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: Trait) , Warn (Target :: Field) , Warn (Target :: Arm) , Warn (Target :: MacroDef) ,]) ; const CREATE : fn (Span) -> AttributeKind = AttributeKind :: Marker ; }
/* FP:traits.rs-0033 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_attr_parsing_src_attributes_traits_STRUCT_0017
/* FP:traits.rs-0034 */ pub (crate) struct DenyExplicitImplParser ;
/* FP:traits.rs-0035 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_attr_parsing_src_attributes_traits_IMPL_0018
/* FP:traits.rs-0036 */ impl < S : Stage > NoArgsAttributeParser < S > for DenyExplicitImplParser { const PATH : & [Symbol] = & [sym :: rustc_deny_explicit_impl] ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Error ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: Trait)]) ; const CREATE : fn (Span) -> AttributeKind = AttributeKind :: DenyExplicitImpl ; }
/* FP:traits.rs-0037 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_attr_parsing_src_attributes_traits_STRUCT_0019
/* FP:traits.rs-0038 */ pub (crate) struct DoNotImplementViaObjectParser ;
/* FP:traits.rs-0039 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_attr_parsing_src_attributes_traits_IMPL_0020
/* FP:traits.rs-0040 */ impl < S : Stage > NoArgsAttributeParser < S > for DoNotImplementViaObjectParser { const PATH : & [Symbol] = & [sym :: rustc_do_not_implement_via_object] ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Error ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: Trait)]) ; const CREATE : fn (Span) -> AttributeKind = AttributeKind :: DoNotImplementViaObject ; }
/* FP:traits.rs-0041 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_attr_parsing_src_attributes_traits_STRUCT_0021
/* FP:traits.rs-0042 */ pub (crate) struct ConstTraitParser ;
/* FP:traits.rs-0043 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_attr_parsing_src_attributes_traits_IMPL_0022
/* FP:traits.rs-0044 */ impl < S : Stage > NoArgsAttributeParser < S > for ConstTraitParser { const PATH : & [Symbol] = & [sym :: const_trait] ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Warn ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: Trait)]) ; const CREATE : fn (Span) -> AttributeKind = AttributeKind :: ConstTrait ; }
/* FP:traits.rs-0045 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_attr_parsing_src_attributes_traits_STRUCT_0023
/* FP:traits.rs-0046 */ pub (crate) struct SpecializationTraitParser ;
/* FP:traits.rs-0047 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_attr_parsing_src_attributes_traits_IMPL_0024
/* FP:traits.rs-0048 */ impl < S : Stage > NoArgsAttributeParser < S > for SpecializationTraitParser { const PATH : & [Symbol] = & [sym :: rustc_specialization_trait] ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Error ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: Trait)]) ; const CREATE : fn (Span) -> AttributeKind = AttributeKind :: SpecializationTrait ; }
/* FP:traits.rs-0049 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_attr_parsing_src_attributes_traits_STRUCT_0025
/* FP:traits.rs-0050 */ pub (crate) struct UnsafeSpecializationMarkerParser ;
/* FP:traits.rs-0051 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_attr_parsing_src_attributes_traits_IMPL_0026
/* FP:traits.rs-0052 */ impl < S : Stage > NoArgsAttributeParser < S > for UnsafeSpecializationMarkerParser { const PATH : & [Symbol] = & [sym :: rustc_unsafe_specialization_marker] ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Error ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: Trait)]) ; const CREATE : fn (Span) -> AttributeKind = AttributeKind :: UnsafeSpecializationMarker ; }
/* FP:traits.rs-0053 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_attr_parsing_src_attributes_traits_STRUCT_0027
/* FP:traits.rs-0054 */ pub (crate) struct CoinductiveParser ;
/* FP:traits.rs-0055 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_attr_parsing_src_attributes_traits_IMPL_0028
/* FP:traits.rs-0056 */ impl < S : Stage > NoArgsAttributeParser < S > for CoinductiveParser { const PATH : & [Symbol] = & [sym :: rustc_coinductive] ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Error ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: Trait)]) ; const CREATE : fn (Span) -> AttributeKind = AttributeKind :: Coinductive ; }
/* FP:traits.rs-0057 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_attr_parsing_src_attributes_traits_STRUCT_0029
/* FP:traits.rs-0058 */ pub (crate) struct AllowIncoherentImplParser ;
/* FP:traits.rs-0059 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_attr_parsing_src_attributes_traits_IMPL_0030
/* FP:traits.rs-0060 */ impl < S : Stage > NoArgsAttributeParser < S > for AllowIncoherentImplParser { const PATH : & [Symbol] = & [sym :: rustc_allow_incoherent_impl] ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Error ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: Method (MethodKind :: Inherent))]) ; const CREATE : fn (Span) -> AttributeKind = AttributeKind :: AllowIncoherentImpl ; }
/* FP:traits.rs-0061 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_attr_parsing_src_attributes_traits_STRUCT_0031
/* FP:traits.rs-0062 */ pub (crate) struct CoherenceIsCoreParser ;
/* FP:traits.rs-0063 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_attr_parsing_src_attributes_traits_IMPL_0032
/* FP:traits.rs-0064 */ impl < S : Stage > NoArgsAttributeParser < S > for CoherenceIsCoreParser { const PATH : & [Symbol] = & [sym :: rustc_coherence_is_core] ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Error ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: Crate)]) ; const TYPE : AttributeType = AttributeType :: CrateLevel ; const CREATE : fn (Span) -> AttributeKind = | _ | AttributeKind :: CoherenceIsCore ; }
/* FP:traits.rs-0065 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_attr_parsing_src_attributes_traits_STRUCT_0033
/* FP:traits.rs-0066 */ pub (crate) struct FundamentalParser ;
/* FP:traits.rs-0067 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_attr_parsing_src_attributes_traits_IMPL_0034
/* FP:traits.rs-0068 */ impl < S : Stage > NoArgsAttributeParser < S > for FundamentalParser { const PATH : & [Symbol] = & [sym :: fundamental] ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Error ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (& [Allow (Target :: Struct) , Allow (Target :: Trait)]) ; const CREATE : fn (Span) -> AttributeKind = | _ | AttributeKind :: Fundamental ; }
/* FP:traits.rs-0069 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_attr_parsing_src_attributes_traits_STRUCT_0035
/* FP:traits.rs-0070 */ pub (crate) struct PointeeParser ;
/* FP:traits.rs-0071 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_attr_parsing_src_attributes_traits_IMPL_0036
/* FP:traits.rs-0072 */ impl < S : Stage > NoArgsAttributeParser < S > for PointeeParser { const PATH : & [Symbol] = & [sym :: pointee] ; const ON_DUPLICATE : OnDuplicate < S > = OnDuplicate :: Error ; const ALLOWED_TARGETS : AllowedTargets = AllowedTargets :: AllowList (ALL_TARGETS) ; const CREATE : fn (Span) -> AttributeKind = AttributeKind :: Pointee ; }