mkuse!{use rustc_macros :: { Decodable , Encodable , HashStable } ;}
mkuse!{use rustc_span :: Symbol ;}
mkuse!{use rustc_span :: def_id :: DefId ;}
mkuse!{use super :: TyCtxt ;}
mkitem!{mkstruct!{# [derive (Copy , Clone , Debug , Decodable , Encodable , HashStable)] pub struct IntrinsicDef { pub name : Symbol , # [doc = " Whether the intrinsic has no meaningful body and all backends need to shim all calls to it."] pub must_be_overridden : bool , # [doc = " Whether the intrinsic can be invoked from stable const fn"] pub const_stable : bool , }}}
mkitem!{mkimpl!{impl TyCtxt < '_ > { pub fn is_intrinsic (self , def_id : DefId , name : Symbol) -> bool { let Some (i) = self . intrinsic (def_id) else { return false } ; i . name == name } }}}