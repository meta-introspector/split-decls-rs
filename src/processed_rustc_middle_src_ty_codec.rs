/* FP:codec.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_codec_USE_0001
/* FP:codec.rs-0002 */ use std :: hash :: Hash ;
/* FP:codec.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_codec_USE_0002
/* FP:codec.rs-0004 */ use std :: intrinsics ;
/* FP:codec.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_codec_USE_0003
/* FP:codec.rs-0006 */ use std :: marker :: { DiscriminantKind , PointeeSized } ;
/* FP:codec.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_codec_USE_0004
/* FP:codec.rs-0008 */ use crate :: rustc_abi :: { FieldIdx , VariantIdx } ;
/* FP:codec.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_codec_USE_0005
/* FP:codec.rs-0010 */ use crate :: rustc_data_structures :: fx :: FxHashMap ;
/* FP:codec.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_codec_USE_0006
/* FP:codec.rs-0012 */ use crate :: rustc_complete :: def_id :: LocalDefId ;
/* FP:codec.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_codec_USE_0007
/* FP:codec.rs-0014 */ use crate :: rustc_serialize :: { Decodable , Encodable } ;
/* FP:codec.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_codec_USE_0008
/* FP:codec.rs-0016 */ use crate :: rustc_complete :: source_map :: Spanned ;
/* FP:codec.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_codec_USE_0009
/* FP:codec.rs-0018 */ use crate :: rustc_complete :: { Span , SpanDecoder , SpanEncoder } ;
/* FP:codec.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_codec_USE_0010
/* FP:codec.rs-0020 */ use crate :: arena :: ArenaAllocatable ;
/* FP:codec.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_codec_USE_0011
/* FP:codec.rs-0022 */ use crate :: infer :: canonical :: { CanonicalVarKind , CanonicalVarKinds } ;
/* FP:codec.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_codec_USE_0012
/* FP:codec.rs-0024 */ use crate :: mir :: interpret :: { AllocId , ConstAllocation , CtfeProvenance } ;
/* FP:codec.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_codec_USE_0013
/* FP:codec.rs-0026 */ use crate :: mir :: mono :: MonoItem ;
/* FP:codec.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_codec_USE_0014
/* FP:codec.rs-0028 */ use crate :: mir :: { self } ;
/* FP:codec.rs-0029 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_codec_USE_0015
/* FP:codec.rs-0030 */ use crate :: traits ;
/* FP:codec.rs-0031 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_codec_USE_0016
/* FP:codec.rs-0032 */ use crate :: ty :: { self , AdtDef , GenericArgsRef , Ty , TyCtxt } ;
/* FP:codec.rs-0033 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_codec_CONST_0017
/* FP:codec.rs-0034 */ # [doc = " The shorthand encoding uses an enum's variant index `usize`"] # [doc = " and is offset by this value so it never matches a real variant."] # [doc = " This offset is also chosen so that the first byte is never < 0x80."] pub const SHORTHAND_OFFSET : usize = 0x80 ;
/* FP:codec.rs-0035 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_codec_TRAIT_0018
/* FP:codec.rs-0036 */ pub trait TyEncoder < 'tcx > : SpanEncoder { const CLEAR_CROSS_CRATE : bool ; fn position (& self) -> usize ; fn type_shorthands (& mut self) -> & mut FxHashMap < Ty < 'tcx > , usize > ; fn predicate_shorthands (& mut self) -> & mut FxHashMap < ty :: PredicateKind < 'tcx > , usize > ; fn encode_alloc_id (& mut self , alloc_id : & AllocId) ; }
/* FP:codec.rs-0037 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_codec_TRAIT_0019
/* FP:codec.rs-0038 */ pub trait TyDecoder < 'tcx > : SpanDecoder { const CLEAR_CROSS_CRATE : bool ; fn interner (& self) -> TyCtxt < 'tcx > ; fn cached_ty_for_shorthand < F > (& mut self , shorthand : usize , or_insert_with : F) -> Ty < 'tcx > where F : FnOnce (& mut Self) -> Ty < 'tcx > ; fn with_position < F , R > (& mut self , pos : usize , f : F) -> R where F : FnOnce (& mut Self) -> R ; fn positioned_at_shorthand (& self) -> bool { (self . peek_byte () & (SHORTHAND_OFFSET as u8)) != 0 } fn decode_alloc_id (& mut self) -> AllocId ; }
/* FP:codec.rs-0039 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_codec_TRAIT_0020
/* FP:codec.rs-0040 */ pub trait EncodableWithShorthand < 'tcx , E : TyEncoder < 'tcx > > : Copy + Eq + Hash { type Variant : Encodable < E > ; fn variant (& self) -> & Self :: Variant ; }
/* FP:codec.rs-0041 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_codec_IMPL_0021
/* FP:codec.rs-0042 */ # [allow (rustc :: usage_of_ty_tykind)] impl < 'tcx , E : TyEncoder < 'tcx > > EncodableWithShorthand < 'tcx , E > for Ty < 'tcx > { type Variant = ty :: TyKind < 'tcx > ; # [inline] fn variant (& self) -> & Self :: Variant { self . kind () } }
/* FP:codec.rs-0043 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_codec_IMPL_0022
/* FP:codec.rs-0044 */ impl < 'tcx , E : TyEncoder < 'tcx > > EncodableWithShorthand < 'tcx , E > for ty :: PredicateKind < 'tcx > { type Variant = ty :: PredicateKind < 'tcx > ; # [inline] fn variant (& self) -> & Self :: Variant { self } }
/* FP:codec.rs-0045 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_codec_TRAIT_0023
/* FP:codec.rs-0046 */ # [doc = " Trait for decoding to a reference."] # [doc = ""] # [doc = " This is a separate trait from `Decodable` so that we can implement it for"] # [doc = " upstream types, such as `FxHashSet`."] # [doc = ""] # [doc = " The `TyDecodable` derive macro will use this trait for fields that are"] # [doc = " references (and don't use a type alias to hide that)."] # [doc = ""] # [doc = " `Decodable` can still be implemented in cases where `Decodable` is required"] # [doc = " by a trait bound."] pub trait RefDecodable < 'tcx , D : TyDecoder < 'tcx > > : PointeeSized { fn decode (d : & mut D) -> & 'tcx Self ; }
/* FP:codec.rs-0047 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_codec_FN_0024
/* FP:codec.rs-0048 */ # [doc = " Encode the given value or a previously cached shorthand."] pub fn encode_with_shorthand < 'tcx , E , T , M > (encoder : & mut E , value : & T , cache : M) where E : TyEncoder < 'tcx > , M : for < 'b > Fn (& 'b mut E) -> & 'b mut FxHashMap < T , usize > , T : EncodableWithShorthand < 'tcx , E > , T :: Variant : DiscriminantKind < Discriminant = isize > , { let existing_shorthand = cache (encoder) . get (value) . copied () ; if let Some (shorthand) = existing_shorthand { encoder . emit_usize (shorthand) ; return ; } let variant = value . variant () ; let start = encoder . position () ; variant . encode (encoder) ; let len = encoder . position () - start ; let discriminant = intrinsics :: discriminant_value (variant) ; assert ! (SHORTHAND_OFFSET > discriminant as usize) ; let shorthand = start + SHORTHAND_OFFSET ; let leb128_bits = len * 7 ; if leb128_bits >= 64 || (shorthand as u64) < (1 << leb128_bits) { cache (encoder) . insert (* value , shorthand) ; } }
/* FP:codec.rs-0049 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_codec_IMPL_0025
/* FP:codec.rs-0050 */ impl < 'tcx , E : TyEncoder < 'tcx > > Encodable < E > for Ty < 'tcx > { fn encode (& self , e : & mut E) { encode_with_shorthand (e , self , TyEncoder :: type_shorthands) ; } }
/* FP:codec.rs-0051 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_codec_IMPL_0026
/* FP:codec.rs-0052 */ impl < 'tcx , E : TyEncoder < 'tcx > > Encodable < E > for ty :: Predicate < 'tcx > { fn encode (& self , e : & mut E) { let kind = self . kind () ; kind . bound_vars () . encode (e) ; encode_with_shorthand (e , & kind . skip_binder () , TyEncoder :: predicate_shorthands) ; } }
/* FP:codec.rs-0053 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_codec_IMPL_0027
/* FP:codec.rs-0054 */ impl < 'tcx , E : TyEncoder < 'tcx > > Encodable < E > for ty :: Clause < 'tcx > { fn encode (& self , e : & mut E) { self . as_predicate () . encode (e) ; } }
/* FP:codec.rs-0055 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_codec_IMPL_0028
/* FP:codec.rs-0056 */ impl < 'tcx , E : TyEncoder < 'tcx > > Encodable < E > for ty :: Region < 'tcx > { fn encode (& self , e : & mut E) { self . kind () . encode (e) ; } }
/* FP:codec.rs-0057 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_codec_IMPL_0029
/* FP:codec.rs-0058 */ impl < 'tcx , E : TyEncoder < 'tcx > > Encodable < E > for ty :: Const < 'tcx > { fn encode (& self , e : & mut E) { self . 0 . 0 . encode (e) ; } }
/* FP:codec.rs-0059 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_codec_IMPL_0030
/* FP:codec.rs-0060 */ impl < 'tcx , E : TyEncoder < 'tcx > > Encodable < E > for ty :: Pattern < 'tcx > { fn encode (& self , e : & mut E) { self . 0 . 0 . encode (e) ; } }
/* FP:codec.rs-0061 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_codec_IMPL_0031
/* FP:codec.rs-0062 */ impl < 'tcx , E : TyEncoder < 'tcx > > Encodable < E > for ty :: ValTree < 'tcx > { fn encode (& self , e : & mut E) { self . 0 . 0 . encode (e) ; } }
/* FP:codec.rs-0063 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_codec_IMPL_0032
/* FP:codec.rs-0064 */ impl < 'tcx , E : TyEncoder < 'tcx > > Encodable < E > for ConstAllocation < 'tcx > { fn encode (& self , e : & mut E) { self . inner () . encode (e) } }
/* FP:codec.rs-0065 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_codec_IMPL_0033
/* FP:codec.rs-0066 */ impl < 'tcx , E : TyEncoder < 'tcx > > Encodable < E > for AdtDef < 'tcx > { fn encode (& self , e : & mut E) { self . 0 . 0 . encode (e) } }
/* FP:codec.rs-0067 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_codec_IMPL_0034
/* FP:codec.rs-0068 */ impl < 'tcx , E : TyEncoder < 'tcx > > Encodable < E > for AllocId { fn encode (& self , e : & mut E) { e . encode_alloc_id (self) } }
/* FP:codec.rs-0069 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_codec_IMPL_0035
/* FP:codec.rs-0070 */ impl < 'tcx , E : TyEncoder < 'tcx > > Encodable < E > for CtfeProvenance { fn encode (& self , e : & mut E) { self . into_parts () . encode (e) ; } }
/* FP:codec.rs-0071 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_codec_IMPL_0036
/* FP:codec.rs-0072 */ impl < 'tcx , E : TyEncoder < 'tcx > > Encodable < E > for ty :: ParamEnv < 'tcx > { fn encode (& self , e : & mut E) { self . caller_bounds () . encode (e) ; } }
/* FP:codec.rs-0073 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_codec_FN_0037
/* FP:codec.rs-0074 */ # [inline] fn decode_arena_allocable < 'tcx , D : TyDecoder < 'tcx > , T : ArenaAllocatable < 'tcx > + Decodable < D > > (decoder : & mut D ,) -> & 'tcx T { decoder . interner () . arena . alloc (Decodable :: decode (decoder)) }
/* FP:codec.rs-0075 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_codec_FN_0038
/* FP:codec.rs-0076 */ # [inline] fn decode_arena_allocable_slice < 'tcx , D : TyDecoder < 'tcx > , T : ArenaAllocatable < 'tcx > + Decodable < D > , > (decoder : & mut D ,) -> & 'tcx [T] { decoder . interner () . arena . alloc_from_iter (< Vec < T > as Decodable < D > > :: decode (decoder)) }
/* FP:codec.rs-0077 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_codec_IMPL_0039
/* FP:codec.rs-0078 */ impl < 'tcx , D : TyDecoder < 'tcx > > Decodable < D > for Ty < 'tcx > { # [allow (rustc :: usage_of_ty_tykind)] fn decode (decoder : & mut D) -> Ty < 'tcx > { if decoder . positioned_at_shorthand () { let pos = decoder . read_usize () ; assert ! (pos >= SHORTHAND_OFFSET) ; let shorthand = pos - SHORTHAND_OFFSET ; decoder . cached_ty_for_shorthand (shorthand , | decoder | { decoder . with_position (shorthand , Ty :: decode) }) } else { let tcx = decoder . interner () ; tcx . mk_ty_from_kind (ty :: TyKind :: decode (decoder)) } } }
/* FP:codec.rs-0079 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_codec_IMPL_0040
/* FP:codec.rs-0080 */ impl < 'tcx , D : TyDecoder < 'tcx > > Decodable < D > for ty :: Predicate < 'tcx > { fn decode (decoder : & mut D) -> ty :: Predicate < 'tcx > { let bound_vars = Decodable :: decode (decoder) ; let predicate_kind = ty :: Binder :: bind_with_vars (if decoder . positioned_at_shorthand () { let pos = decoder . read_usize () ; assert ! (pos >= SHORTHAND_OFFSET) ; let shorthand = pos - SHORTHAND_OFFSET ; decoder . with_position (shorthand , < ty :: PredicateKind < 'tcx > as Decodable < D > > :: decode) } else { < ty :: PredicateKind < 'tcx > as Decodable < D > > :: decode (decoder) } , bound_vars ,) ; decoder . interner () . mk_predicate (predicate_kind) } }
/* FP:codec.rs-0081 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_codec_IMPL_0041
/* FP:codec.rs-0082 */ impl < 'tcx , D : TyDecoder < 'tcx > > Decodable < D > for ty :: Clause < 'tcx > { fn decode (decoder : & mut D) -> ty :: Clause < 'tcx > { let pred : ty :: Predicate < 'tcx > = Decodable :: decode (decoder) ; pred . expect_clause () } }
/* FP:codec.rs-0083 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_codec_IMPL_0042
/* FP:codec.rs-0084 */ impl < 'tcx , D : TyDecoder < 'tcx > > Decodable < D > for GenericArgsRef < 'tcx > { fn decode (decoder : & mut D) -> Self { let len = decoder . read_usize () ; let tcx = decoder . interner () ; tcx . mk_args_from_iter ((0 .. len) . map :: < ty :: GenericArg < 'tcx > , _ > (| _ | Decodable :: decode (decoder)) ,) } }
/* FP:codec.rs-0085 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_codec_IMPL_0043
/* FP:codec.rs-0086 */ impl < 'tcx , D : TyDecoder < 'tcx > > Decodable < D > for mir :: Place < 'tcx > { fn decode (decoder : & mut D) -> Self { let local : mir :: Local = Decodable :: decode (decoder) ; let len = decoder . read_usize () ; let projection = decoder . interner () . mk_place_elems_from_iter ((0 .. len) . map :: < mir :: PlaceElem < 'tcx > , _ > (| _ | Decodable :: decode (decoder)) ,) ; mir :: Place { local , projection } } }
/* FP:codec.rs-0087 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_codec_IMPL_0044
/* FP:codec.rs-0088 */ impl < 'tcx , D : TyDecoder < 'tcx > > Decodable < D > for ty :: Region < 'tcx > { fn decode (decoder : & mut D) -> Self { ty :: Region :: new_from_kind (decoder . interner () , Decodable :: decode (decoder)) } }
/* FP:codec.rs-0089 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_codec_IMPL_0045
/* FP:codec.rs-0090 */ impl < 'tcx , D : TyDecoder < 'tcx > > Decodable < D > for CanonicalVarKinds < 'tcx > { fn decode (decoder : & mut D) -> Self { let len = decoder . read_usize () ; decoder . interner () . mk_canonical_var_infos_from_iter ((0 .. len) . map :: < CanonicalVarKind < 'tcx > , _ > (| _ | Decodable :: decode (decoder)) ,) } }
/* FP:codec.rs-0091 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_codec_IMPL_0046
/* FP:codec.rs-0092 */ impl < 'tcx , D : TyDecoder < 'tcx > > Decodable < D > for AllocId { fn decode (decoder : & mut D) -> Self { decoder . decode_alloc_id () } }
/* FP:codec.rs-0093 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_codec_IMPL_0047
/* FP:codec.rs-0094 */ impl < 'tcx , D : TyDecoder < 'tcx > > Decodable < D > for CtfeProvenance { fn decode (decoder : & mut D) -> Self { let parts = Decodable :: decode (decoder) ; CtfeProvenance :: from_parts (parts) } }
/* FP:codec.rs-0095 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_codec_IMPL_0048
/* FP:codec.rs-0096 */ impl < 'tcx , D : TyDecoder < 'tcx > > Decodable < D > for ty :: SymbolName < 'tcx > { fn decode (decoder : & mut D) -> Self { ty :: SymbolName :: new (decoder . interner () , decoder . read_str ()) } }
/* FP:codec.rs-0097 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_codec_IMPL_0049
/* FP:codec.rs-0098 */ impl < 'tcx , D : TyDecoder < 'tcx > > Decodable < D > for ty :: ParamEnv < 'tcx > { fn decode (d : & mut D) -> Self { let caller_bounds = Decodable :: decode (d) ; ty :: ParamEnv :: new (caller_bounds) } }
/* FP:codec.rs-0099 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_codec_MACRO_0050
/* FP:codec.rs-0100 */ macro_rules ! impl_decodable_via_ref { ($ ($ t : ty ,) +) => { $ (impl <'tcx , D : TyDecoder <'tcx >> Decodable < D > for $ t { fn decode (decoder : & mut D) -> Self { RefDecodable :: decode (decoder) } }) * } }
/* FP:codec.rs-0101 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_codec_IMPL_0051
/* FP:codec.rs-0102 */ impl < 'tcx , D : TyDecoder < 'tcx > > RefDecodable < 'tcx , D > for ty :: List < Ty < 'tcx > > { fn decode (decoder : & mut D) -> & 'tcx Self { let len = decoder . read_usize () ; decoder . interner () . mk_type_list_from_iter ((0 .. len) . map :: < Ty < 'tcx > , _ > (| _ | Decodable :: decode (decoder))) } }
/* FP:codec.rs-0103 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_codec_IMPL_0052
/* FP:codec.rs-0104 */ impl < 'tcx , D : TyDecoder < 'tcx > > RefDecodable < 'tcx , D > for ty :: List < ty :: PolyExistentialPredicate < 'tcx > > { fn decode (decoder : & mut D) -> & 'tcx Self { let len = decoder . read_usize () ; decoder . interner () . mk_poly_existential_predicates_from_iter ((0 .. len) . map :: < ty :: Binder < 'tcx , _ > , _ > (| _ | Decodable :: decode (decoder)) ,) } }
/* FP:codec.rs-0105 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_codec_IMPL_0053
/* FP:codec.rs-0106 */ impl < 'tcx , D : TyDecoder < 'tcx > > Decodable < D > for ty :: Const < 'tcx > { fn decode (decoder : & mut D) -> Self { let kind : ty :: ConstKind < 'tcx > = Decodable :: decode (decoder) ; decoder . interner () . mk_ct_from_kind (kind) } }
/* FP:codec.rs-0107 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_codec_IMPL_0054
/* FP:codec.rs-0108 */ impl < 'tcx , D : TyDecoder < 'tcx > > Decodable < D > for ty :: Pattern < 'tcx > { fn decode (decoder : & mut D) -> Self { decoder . interner () . mk_pat (Decodable :: decode (decoder)) } }
/* FP:codec.rs-0109 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_codec_IMPL_0055
/* FP:codec.rs-0110 */ impl < 'tcx , D : TyDecoder < 'tcx > > Decodable < D > for ty :: ValTree < 'tcx > { fn decode (decoder : & mut D) -> Self { decoder . interner () . intern_valtree (Decodable :: decode (decoder)) } }
/* FP:codec.rs-0111 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_codec_IMPL_0056
/* FP:codec.rs-0112 */ impl < 'tcx , D : TyDecoder < 'tcx > > Decodable < D > for ConstAllocation < 'tcx > { fn decode (decoder : & mut D) -> Self { decoder . interner () . mk_const_alloc (Decodable :: decode (decoder)) } }
/* FP:codec.rs-0113 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_codec_IMPL_0057
/* FP:codec.rs-0114 */ impl < 'tcx , D : TyDecoder < 'tcx > > Decodable < D > for AdtDef < 'tcx > { fn decode (decoder : & mut D) -> Self { decoder . interner () . mk_adt_def_from_data (Decodable :: decode (decoder)) } }
/* FP:codec.rs-0115 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_codec_IMPL_0058
/* FP:codec.rs-0116 */ impl < 'tcx , D : TyDecoder < 'tcx > > RefDecodable < 'tcx , D > for [(ty :: Clause < 'tcx > , Span)] { fn decode (decoder : & mut D) -> & 'tcx Self { decoder . interner () . arena . alloc_from_iter ((0 .. decoder . read_usize ()) . map (| _ | Decodable :: decode (decoder))) } }
/* FP:codec.rs-0117 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_codec_IMPL_0059
/* FP:codec.rs-0118 */ impl < 'tcx , D : TyDecoder < 'tcx > > RefDecodable < 'tcx , D > for [(ty :: PolyTraitRef < 'tcx > , Span)] { fn decode (decoder : & mut D) -> & 'tcx Self { decoder . interner () . arena . alloc_from_iter ((0 .. decoder . read_usize ()) . map (| _ | Decodable :: decode (decoder))) } }
/* FP:codec.rs-0119 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_codec_IMPL_0060
/* FP:codec.rs-0120 */ impl < 'tcx , D : TyDecoder < 'tcx > > RefDecodable < 'tcx , D > for [Spanned < MonoItem < 'tcx > >] { fn decode (decoder : & mut D) -> & 'tcx Self { decoder . interner () . arena . alloc_from_iter ((0 .. decoder . read_usize ()) . map (| _ | Decodable :: decode (decoder))) } }
/* FP:codec.rs-0121 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_codec_IMPL_0061
/* FP:codec.rs-0122 */ impl < 'tcx , D : TyDecoder < 'tcx > > RefDecodable < 'tcx , D > for ty :: List < ty :: BoundVariableKind > { fn decode (decoder : & mut D) -> & 'tcx Self { let len = decoder . read_usize () ; decoder . interner () . mk_bound_variable_kinds_from_iter ((0 .. len) . map :: < ty :: BoundVariableKind , _ > (| _ | Decodable :: decode (decoder)) ,) } }
/* FP:codec.rs-0123 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_codec_IMPL_0062
/* FP:codec.rs-0124 */ impl < 'tcx , D : TyDecoder < 'tcx > > RefDecodable < 'tcx , D > for ty :: List < ty :: Pattern < 'tcx > > { fn decode (decoder : & mut D) -> & 'tcx Self { let len = decoder . read_usize () ; decoder . interner () . mk_patterns_from_iter ((0 .. len) . map :: < ty :: Pattern < 'tcx > , _ > (| _ | Decodable :: decode (decoder)) ,) } }
/* FP:codec.rs-0125 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_codec_IMPL_0063
/* FP:codec.rs-0126 */ impl < 'tcx , D : TyDecoder < 'tcx > > RefDecodable < 'tcx , D > for ty :: List < ty :: Const < 'tcx > > { fn decode (decoder : & mut D) -> & 'tcx Self { let len = decoder . read_usize () ; decoder . interner () . mk_const_list_from_iter ((0 .. len) . map :: < ty :: Const < 'tcx > , _ > (| _ | Decodable :: decode (decoder)) ,) } }
/* FP:codec.rs-0127 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_codec_IMPL_0064
/* FP:codec.rs-0128 */ impl < 'tcx , D : TyDecoder < 'tcx > > RefDecodable < 'tcx , D > for ty :: ListWithCachedTypeInfo < ty :: Clause < 'tcx > > { fn decode (decoder : & mut D) -> & 'tcx Self { let len = decoder . read_usize () ; decoder . interner () . mk_clauses_from_iter ((0 .. len) . map :: < ty :: Clause < 'tcx > , _ > (| _ | Decodable :: decode (decoder)) ,) } }
/* FP:codec.rs-0129 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_codec_IMPL_0065
/* FP:codec.rs-0130 */ impl < 'tcx , D : TyDecoder < 'tcx > > RefDecodable < 'tcx , D > for ty :: List < FieldIdx > { fn decode (decoder : & mut D) -> & 'tcx Self { let len = decoder . read_usize () ; decoder . interner () . mk_fields_from_iter ((0 .. len) . map :: < FieldIdx , _ > (| _ | Decodable :: decode (decoder))) } }
/* FP:codec.rs-0131 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_codec_IMPL_0066
/* FP:codec.rs-0132 */ impl < 'tcx , D : TyDecoder < 'tcx > > RefDecodable < 'tcx , D > for ty :: List < LocalDefId > { fn decode (decoder : & mut D) -> & 'tcx Self { let len = decoder . read_usize () ; decoder . interner () . mk_local_def_ids_from_iter ((0 .. len) . map :: < LocalDefId , _ > (| _ | Decodable :: decode (decoder)) ,) } }
/* FP:codec.rs-0133 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_codec_IMPL_0067
/* FP:codec.rs-0134 */ impl < 'tcx , D : TyDecoder < 'tcx > > Decodable < D > for & 'tcx ty :: List < LocalDefId > { fn decode (d : & mut D) -> Self { RefDecodable :: decode (d) } }
/* FP:codec.rs-0135 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_codec_IMPL_0068
/* FP:codec.rs-0136 */ impl < 'tcx , D : TyDecoder < 'tcx > > RefDecodable < 'tcx , D > for ty :: List < (VariantIdx , FieldIdx) > { fn decode (decoder : & mut D) -> & 'tcx Self { let len = decoder . read_usize () ; decoder . interner () . mk_offset_of_from_iter ((0 .. len) . map :: < (VariantIdx , FieldIdx) , _ > (| _ | Decodable :: decode (decoder)) ,) } }
/* FP:codec.rs-0137 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_codec_MACRO_0069
/* FP:codec.rs-0138 */ impl_decodable_via_ref ! { &'tcx ty :: TypeckResults <'tcx >, &'tcx ty :: List < Ty <'tcx >>, &'tcx ty :: List < ty :: PolyExistentialPredicate <'tcx >>, &'tcx traits :: ImplSource <'tcx , () >, &'tcx mir :: Body <'tcx >, &'tcx ty :: List < ty :: BoundVariableKind >, &'tcx ty :: List < ty :: Pattern <'tcx >>, &'tcx ty :: ListWithCachedTypeInfo < ty :: Clause <'tcx >>, }
/* FP:codec.rs-0139 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_codec_MACRO_0070
/* FP:codec.rs-0140 */ # [macro_export] macro_rules ! __impl_decoder_methods { ($ ($ name : ident -> $ ty : ty ;) *) => { $ (# [inline] fn $ name (& mut self) -> $ ty { self . opaque .$ name () }) * } }
/* FP:codec.rs-0141 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_codec_MACRO_0071
/* FP:codec.rs-0142 */ macro_rules ! impl_arena_allocatable_decoder { ([] $ args : tt) => { } ; ([decode $ (, $ attrs : ident) *] [$ name : ident : $ ty : ty]) => { impl <'tcx , D : TyDecoder <'tcx >> RefDecodable <'tcx , D > for $ ty { # [inline] fn decode (decoder : & mut D) -> &'tcx Self { decode_arena_allocable (decoder) } } impl <'tcx , D : TyDecoder <'tcx >> RefDecodable <'tcx , D > for [$ ty] { # [inline] fn decode (decoder : & mut D) -> &'tcx Self { decode_arena_allocable_slice (decoder) } } } ; }
/* FP:codec.rs-0143 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_codec_MACRO_0072
/* FP:codec.rs-0144 */ macro_rules ! impl_arena_allocatable_decoders { ([$ ($ a : tt $ name : ident : $ ty : ty ,) *]) => { $ (impl_arena_allocatable_decoder ! ($ a [$ name : $ ty]) ;) * } }
/* FP:codec.rs-0145 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_codec_MACRO_0073
/* FP:codec.rs-0146 */ crate :: rustc_hir :: arena_types ! (impl_arena_allocatable_decoders) ;
/* FP:codec.rs-0147 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_codec_MACRO_0074
/* FP:codec.rs-0148 */ arena_types ! (impl_arena_allocatable_decoders) ;
/* FP:codec.rs-0149 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_codec_MACRO_0075
/* FP:codec.rs-0150 */ macro_rules ! impl_arena_copy_decoder { (<$ tcx : tt > $ ($ ty : ty ,) *) => { $ (impl <'tcx , D : TyDecoder <'tcx >> RefDecodable <'tcx , D > for $ ty { # [inline] fn decode (decoder : & mut D) -> &'tcx Self { decoder . interner () . arena . alloc (Decodable :: decode (decoder)) } } impl <'tcx , D : TyDecoder <'tcx >> RefDecodable <'tcx , D > for [$ ty] { # [inline] fn decode (decoder : & mut D) -> &'tcx Self { decoder . interner () . arena . alloc_from_iter (< Vec < _ > as Decodable < D >>:: decode (decoder)) } }) * } ; }
/* FP:codec.rs-0151 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_codec_MACRO_0076
/* FP:codec.rs-0152 */ impl_arena_copy_decoder ! { <'tcx > Span , crate :: rustc_span :: Ident , ty :: Variance , crate :: rustc_span :: def_id :: DefId , crate :: rustc_span :: def_id :: LocalDefId , (crate :: rustc_middle :: middle :: exported_symbols :: ExportedSymbol <'tcx >, crate :: rustc_middle :: middle :: exported_symbols :: SymbolExportInfo) , ty :: DeducedParamAttrs , }
/* FP:codec.rs-0153 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_ty_codec_MACRO_0077