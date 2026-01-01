/* FP:resolve_bound_vars.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_middle_resolve_bound_vars_USE_0001
/* FP:resolve_bound_vars.rs-0002 */ use crate :: rustc_data_structures :: sorted_map :: SortedMap ;
/* FP:resolve_bound_vars.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_middle_resolve_bound_vars_USE_0002
/* FP:resolve_bound_vars.rs-0004 */ use crate :: rustc_complete :: ErrorGuaranteed ;
/* FP:resolve_bound_vars.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_middle_resolve_bound_vars_USE_0003
/* FP:resolve_bound_vars.rs-0006 */ use crate :: rustc_complete :: ItemLocalId ;
/* FP:resolve_bound_vars.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_middle_resolve_bound_vars_USE_0004
/* FP:resolve_bound_vars.rs-0008 */ use crate :: rustc_complete :: def_id :: { DefId , LocalDefId , LocalDefIdMap } ;
/* FP:resolve_bound_vars.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_middle_resolve_bound_vars_USE_0005
/* FP:resolve_bound_vars.rs-0010 */ use rustc_macros :: { Decodable , Encodable , HashStable , TyDecodable , TyEncodable } ;
/* FP:resolve_bound_vars.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_middle_resolve_bound_vars_USE_0006
/* FP:resolve_bound_vars.rs-0012 */ use crate :: ty ;
/* FP:resolve_bound_vars.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_middle_resolve_bound_vars_ENUM_0007
/* FP:resolve_bound_vars.rs-0014 */ # [derive (Clone , Copy , PartialEq , Eq , Hash , TyEncodable , TyDecodable , Debug , HashStable)] pub enum ResolvedArg { StaticLifetime , EarlyBound (LocalDefId) , LateBound (ty :: DebruijnIndex , u32 , LocalDefId) , Free (LocalDefId , LocalDefId) , Error (ErrorGuaranteed) , }
/* FP:resolve_bound_vars.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_middle_resolve_bound_vars_ENUM_0008
/* FP:resolve_bound_vars.rs-0016 */ # [doc = " A set containing, at most, one known element."] # [doc = " If two distinct values are inserted into a set, then it"] # [doc = " becomes `Many`, which can be used to detect ambiguities."] # [derive (Copy , Clone , PartialEq , Eq , TyEncodable , TyDecodable , Debug , HashStable)] pub enum Set1 < T > { Empty , One (T) , Many , }
/* FP:resolve_bound_vars.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_middle_resolve_bound_vars_IMPL_0009
/* FP:resolve_bound_vars.rs-0018 */ impl < T : PartialEq > Set1 < T > { pub fn insert (& mut self , value : T) { * self = match self { Set1 :: Empty => Set1 :: One (value) , Set1 :: One (old) if * old == value => return , _ => Set1 :: Many , } ; } }
/* FP:resolve_bound_vars.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_middle_resolve_bound_vars_ENUM_0010
/* FP:resolve_bound_vars.rs-0020 */ # [derive (Copy , Clone , Debug , HashStable , Encodable , Decodable)] pub enum ObjectLifetimeDefault { Empty , Static , Ambiguous , Param (DefId) , }
/* FP:resolve_bound_vars.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_middle_src_middle_resolve_bound_vars_STRUCT_0011
/* FP:resolve_bound_vars.rs-0022 */ # [doc = " Maps the id of each bound variable reference to the variable decl"] # [doc = " that it corresponds to."] # [derive (Debug , Default , HashStable)] pub struct ResolveBoundVars { pub defs : SortedMap < ItemLocalId , ResolvedArg > , pub late_bound_vars : SortedMap < ItemLocalId , Vec < ty :: BoundVariableKind > > , pub opaque_captured_lifetimes : LocalDefIdMap < Vec < (ResolvedArg , LocalDefId) > > , }