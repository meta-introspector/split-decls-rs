mkuse!{use std :: fmt :: Debug ;}
mkuse!{use super :: context :: CompilerCtxt ;}
mkuse!{use super :: { Bridge , Tables } ;}
mkitem!{mktrait!{pub trait Error { fn new (msg : String) -> Self ; fn from_internal < T : Debug > (err : T) -> Self ; }}}
mkitem!{mktrait!{pub trait Prov < B : Bridge > { fn new (aid : B :: AllocId) -> Self ; }}}
mkitem!{mktrait!{pub trait Allocation < B : Bridge > { fn new < 'tcx > (bytes : Vec < Option < u8 > > , ptrs : Vec < (usize , rustc_middle :: mir :: interpret :: AllocId) > , align : u64 , mutability : rustc_middle :: mir :: Mutability , tables : & mut Tables < 'tcx , B > , cx : & CompilerCtxt < 'tcx , B > ,) -> Self ; }}}
mkitem!{macro_rules ! make_bridge_trait { ($ name : ident) => { pub trait $ name < B : Bridge > { fn new (did : B :: DefId) -> Self ; } } ; }}
mkitem!{make_bridge_trait ! (CrateItem) ;}
mkitem!{make_bridge_trait ! (AdtDef) ;}
mkitem!{make_bridge_trait ! (ForeignModuleDef) ;}
mkitem!{make_bridge_trait ! (ForeignDef) ;}
mkitem!{make_bridge_trait ! (FnDef) ;}
mkitem!{make_bridge_trait ! (ClosureDef) ;}
mkitem!{make_bridge_trait ! (CoroutineDef) ;}
mkitem!{make_bridge_trait ! (CoroutineClosureDef) ;}
mkitem!{make_bridge_trait ! (AliasDef) ;}
mkitem!{make_bridge_trait ! (ParamDef) ;}
mkitem!{make_bridge_trait ! (BrNamedDef) ;}
mkitem!{make_bridge_trait ! (TraitDef) ;}
mkitem!{make_bridge_trait ! (GenericDef) ;}
mkitem!{make_bridge_trait ! (ConstDef) ;}
mkitem!{make_bridge_trait ! (ImplDef) ;}
mkitem!{make_bridge_trait ! (RegionDef) ;}
mkitem!{make_bridge_trait ! (CoroutineWitnessDef) ;}
mkitem!{make_bridge_trait ! (AssocDef) ;}
mkitem!{make_bridge_trait ! (OpaqueDef) ;}
mkitem!{make_bridge_trait ! (StaticDef) ;}