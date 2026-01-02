mkuse!{use rustc_data_structures :: sorted_map :: SortedMap ;}
mkuse!{use rustc_errors :: ErrorGuaranteed ;}
mkuse!{use rustc_hir :: ItemLocalId ;}
mkuse!{use rustc_hir :: def_id :: { DefId , LocalDefId , LocalDefIdMap } ;}
mkuse!{use rustc_macros :: { Decodable , Encodable , HashStable , TyDecodable , TyEncodable } ;}
mkuse!{use crate :: ty ;}
mkitem!{mkenum!{# [derive (Clone , Copy , PartialEq , Eq , Hash , TyEncodable , TyDecodable , Debug , HashStable)] pub enum ResolvedArg { StaticLifetime , EarlyBound (LocalDefId) , LateBound (ty :: DebruijnIndex , u32 , LocalDefId) , Free (LocalDefId , LocalDefId) , Error (ErrorGuaranteed) , }}}
mkitem!{mkenum!{# [doc = " A set containing, at most, one known element."] # [doc = " If two distinct values are inserted into a set, then it"] # [doc = " becomes `Many`, which can be used to detect ambiguities."] # [derive (Copy , Clone , PartialEq , Eq , TyEncodable , TyDecodable , Debug , HashStable)] pub enum Set1 < T > { Empty , One (T) , Many , }}}
mkitem!{mkimpl!{impl < T : PartialEq > Set1 < T > { pub fn insert (& mut self , value : T) { * self = match self { Set1 :: Empty => Set1 :: One (value) , Set1 :: One (old) if * old == value => return , _ => Set1 :: Many , } ; } }}}
mkitem!{mkenum!{# [derive (Copy , Clone , Debug , HashStable , Encodable , Decodable)] pub enum ObjectLifetimeDefault { Empty , Static , Ambiguous , Param (DefId) , }}}
mkitem!{mkstruct!{# [doc = " Maps the id of each bound variable reference to the variable decl"] # [doc = " that it corresponds to."] # [derive (Debug , Default , HashStable)] pub struct ResolveBoundVars { pub defs : SortedMap < ItemLocalId , ResolvedArg > , pub late_bound_vars : SortedMap < ItemLocalId , Vec < ty :: BoundVariableKind > > , pub opaque_captured_lifetimes : LocalDefIdMap < Vec < (ResolvedArg , LocalDefId) > > , }}}