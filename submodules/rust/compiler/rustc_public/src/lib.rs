mkuse!{use std :: fmt :: Debug ;}
mkuse!{use std :: { fmt , io } ;}
mkuse!{pub (crate) use rustc_public_bridge :: IndexedVal ;}
mkuse!{use rustc_public_bridge :: Tables ;}
mkuse!{use rustc_public_bridge :: context :: CompilerCtxt ;}
mkmod!{rustc_internal, { 
                getname!(rustc_internal);
                getsrc!(rustc_internal);
                getpath!(rustc_internal);
                get_deps!(rustc_internal);
                get_crates!(rustc_internal);
                mkinclude!(rustc_internal);
                 
            }}
mkuse!{use serde :: Serialize ;}
mkuse!{use crate :: compiler_interface :: with ;}
mkuse!{pub use crate :: crate_def :: { CrateDef , CrateDefItems , CrateDefType , DefId } ;}
mkuse!{pub use crate :: error :: * ;}
mkuse!{use crate :: mir :: mono :: StaticDef ;}
mkuse!{use crate :: mir :: { Body , Mutability } ;}
mkuse!{use crate :: ty :: { AssocItem , FnDef , ForeignModuleDef , ImplDef , ProvenanceMap , Span , TraitDef , Ty } ;}
mkuse!{use crate :: unstable :: Stable ;}
mkmod!{abi, { 
                getname!(abi);
                getsrc!(abi);
                getpath!(abi);
                get_deps!(abi);
                get_crates!(abi);
                mkinclude!(abi);
                 
            }}
mkmod!{alloc, { 
                getname!(alloc);
                getsrc!(alloc);
                getpath!(alloc);
                get_deps!(alloc);
                get_crates!(alloc);
                mkinclude!(alloc);
                 
            }}
mkmod!{unstable, { 
                getname!(unstable);
                getsrc!(unstable);
                getpath!(unstable);
                get_deps!(unstable);
                get_crates!(unstable);
                mkinclude!(unstable);
                 
            }}
mkmod!{crate_def, { 
                getname!(crate_def);
                getsrc!(crate_def);
                getpath!(crate_def);
                get_deps!(crate_def);
                get_crates!(crate_def);
                mkinclude!(crate_def);
                 
            }}
mkmod!{compiler_interface, { 
                getname!(compiler_interface);
                getsrc!(compiler_interface);
                getpath!(compiler_interface);
                get_deps!(compiler_interface);
                get_crates!(compiler_interface);
                mkinclude!(compiler_interface);
                 
            }}
mkmod!{error, { 
                getname!(error);
                getsrc!(error);
                getpath!(error);
                get_deps!(error);
                get_crates!(error);
                mkinclude!(error);
                 
            }}
mkmod!{mir, { 
                getname!(mir);
                getsrc!(mir);
                getpath!(mir);
                get_deps!(mir);
                get_crates!(mir);
                mkinclude!(mir);
                 
            }}
mkmod!{target, { 
                getname!(target);
                getsrc!(target);
                getpath!(target);
                get_deps!(target);
                get_crates!(target);
                mkinclude!(target);
                 
            }}
mkmod!{ty, { 
                getname!(ty);
                getsrc!(ty);
                getpath!(ty);
                get_deps!(ty);
                get_crates!(ty);
                mkinclude!(ty);
                 
            }}
mkmod!{visitor, { 
                getname!(visitor);
                getsrc!(visitor);
                getpath!(visitor);
                get_deps!(visitor);
                get_crates!(visitor);
                mkinclude!(visitor);
                 
            }}
mkitem!{# [doc = " Use String for now but we should replace it."] pub type Symbol = String ;}
mkitem!{# [doc = " The number that identifies a crate."] pub type CrateNum = usize ;}
mkitem!{mkimpl!{impl Debug for DefId { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("DefId") . field ("id" , & self . 0) . field ("name" , & self . name ()) . finish () } }}}
mkitem!{mkimpl!{impl IndexedVal for DefId { fn to_val (index : usize) -> Self { DefId (index) } fn to_index (& self) -> usize { self . 0 } }}}
mkitem!{# [doc = " A list of crate items."] pub type CrateItems = Vec < CrateItem > ;}
mkitem!{# [doc = " A list of trait decls."] pub type TraitDecls = Vec < TraitDef > ;}
mkitem!{# [doc = " A list of impl trait decls."] pub type ImplTraitDecls = Vec < ImplDef > ;}
mkitem!{# [doc = " A list of associated items."] pub type AssocItems = Vec < AssocItem > ;}
mkitem!{mkstruct!{# [doc = " Holds information about a crate."] # [derive (Clone , PartialEq , Eq , Debug , Serialize)] pub struct Crate { pub id : CrateNum , pub name : Symbol , pub is_local : bool , }}}
mkitem!{mkimpl!{impl Crate { # [doc = " The list of foreign modules in this crate."] pub fn foreign_modules (& self) -> Vec < ForeignModuleDef > { with (| cx | cx . foreign_modules (self . id)) } # [doc = " The list of traits declared in this crate."] pub fn trait_decls (& self) -> TraitDecls { with (| cx | cx . trait_decls (self . id)) } # [doc = " The list of trait implementations in this crate."] pub fn trait_impls (& self) -> ImplTraitDecls { with (| cx | cx . trait_impls (self . id)) } # [doc = " Return a list of function definitions from this crate independent on their visibility."] pub fn fn_defs (& self) -> Vec < FnDef > { with (| cx | cx . crate_functions (self . id)) } # [doc = " Return a list of static items defined in this crate independent on their visibility."] pub fn statics (& self) -> Vec < StaticDef > { with (| cx | cx . crate_statics (self . id)) } }}}
mkitem!{mkenum!{# [derive (Copy , Clone , PartialEq , Eq , Debug , Hash , Serialize)] pub enum ItemKind { Fn , Static , Const , Ctor (CtorKind) , }}}
mkitem!{mkenum!{# [derive (Copy , Clone , PartialEq , Eq , Debug , Hash , Serialize)] pub enum CtorKind { Const , Fn , }}}
mkitem!{pub type Filename = String ;}
mkitem!{crate_def_with_ty ! { # [doc = " Holds information about an item in a crate."] # [derive (Serialize)] pub CrateItem ; }}
mkitem!{mkimpl!{impl CrateItem { # [doc = " This will return the body of an item or panic if it's not available."] pub fn expect_body (& self) -> mir :: Body { with (| cx | cx . mir_body (self . 0)) } # [doc = " Return the body of an item if available."] pub fn body (& self) -> Option < mir :: Body > { with (| cx | cx . has_body (self . 0) . then (| | cx . mir_body (self . 0))) } # [doc = " Check if a body is available for this item."] pub fn has_body (& self) -> bool { with (| cx | cx . has_body (self . 0)) } pub fn span (& self) -> Span { with (| cx | cx . span_of_an_item (self . 0)) } pub fn kind (& self) -> ItemKind { with (| cx | cx . item_kind (* self)) } pub fn requires_monomorphization (& self) -> bool { with (| cx | cx . requires_monomorphization (self . 0)) } pub fn ty (& self) -> Ty { with (| cx | cx . def_ty (self . 0)) } pub fn is_foreign_item (& self) -> bool { with (| cx | cx . is_foreign_item (self . 0)) } # [doc = " Emit MIR for this item body."] pub fn emit_mir < W : io :: Write > (& self , w : & mut W) -> io :: Result < () > { self . body () . ok_or_else (| | io :: Error :: other (format ! ("No body found for `{}`" , self . name ()))) ? . dump (w , & self . name ()) } }}}

macro_rules! entry_fn_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function entry_fn in module {}", module_path!());
    };
}

mkfn!{
    entry_fn_introspect!();
    # [doc = " Return the function where execution starts if the current"] # [doc = " crate defines that. This is usually `main`, but could be"] # [doc = " `start` if the crate is a no-std crate."] pub fn entry_fn () -> Option < CrateItem > { with (| cx | cx . entry_fn ()) }
}

macro_rules! local_crate_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function local_crate in module {}", module_path!());
    };
}

mkfn!{
    local_crate_introspect!();
    # [doc = " Access to the local crate."] pub fn local_crate () -> Crate { with (| cx | cx . local_crate ()) }
}

macro_rules! find_crates_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function find_crates in module {}", module_path!());
    };
}

mkfn!{
    find_crates_introspect!();
    # [doc = " Try to find a crate or crates if multiple crates exist from given name."] pub fn find_crates (name : & str) -> Vec < Crate > { with (| cx | cx . find_crates (name)) }
}

macro_rules! external_crates_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function external_crates in module {}", module_path!());
    };
}

mkfn!{
    external_crates_introspect!();
    # [doc = " Try to find a crate with the given name."] pub fn external_crates () -> Vec < Crate > { with (| cx | cx . external_crates ()) }
}

macro_rules! all_local_items_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function all_local_items in module {}", module_path!());
    };
}

mkfn!{
    all_local_items_introspect!();
    # [doc = " Retrieve all items in the local crate that have a MIR associated with them."] pub fn all_local_items () -> CrateItems { with (| cx | cx . all_local_items ()) }
}

macro_rules! all_trait_decls_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function all_trait_decls in module {}", module_path!());
    };
}

mkfn!{
    all_trait_decls_introspect!();
    pub fn all_trait_decls () -> TraitDecls { with (| cx | cx . all_trait_decls ()) }
}

macro_rules! all_trait_impls_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function all_trait_impls in module {}", module_path!());
    };
}

mkfn!{
    all_trait_impls_introspect!();
    pub fn all_trait_impls () -> ImplTraitDecls { with (| cx | cx . all_trait_impls ()) }
}
mkitem!{mkstruct!{# [doc = " A type that provides internal information but that can still be used for debug purpose."] # [derive (Clone , PartialEq , Eq , Hash , Serialize)] pub struct Opaque (String) ;}}
mkitem!{mkimpl!{impl std :: fmt :: Display for Opaque { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { write ! (f , "{}" , self . 0) } }}}
mkitem!{mkimpl!{impl std :: fmt :: Debug for Opaque { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { write ! (f , "{}" , self . 0) } }}}

macro_rules! opaque_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function opaque in module {}", module_path!());
    };
}

mkfn!{
    opaque_introspect!();
    pub fn opaque < T : Debug > (value : & T) -> Opaque { Opaque (format ! ("{value:?}")) }
}
mkitem!{macro_rules ! bridge_impl { ($ name : ident , $ ty : ty) => { impl rustc_public_bridge :: bridge ::$ name < compiler_interface :: BridgeTys > for $ ty { fn new (def : crate :: DefId) -> Self { Self (def) } } } ; }}
mkitem!{bridge_impl ! (CrateItem , crate :: CrateItem) ;}
mkitem!{bridge_impl ! (AdtDef , crate :: ty :: AdtDef) ;}
mkitem!{bridge_impl ! (ForeignModuleDef , crate :: ty :: ForeignModuleDef) ;}
mkitem!{bridge_impl ! (ForeignDef , crate :: ty :: ForeignDef) ;}
mkitem!{bridge_impl ! (FnDef , crate :: ty :: FnDef) ;}
mkitem!{bridge_impl ! (ClosureDef , crate :: ty :: ClosureDef) ;}
mkitem!{bridge_impl ! (CoroutineDef , crate :: ty :: CoroutineDef) ;}
mkitem!{bridge_impl ! (CoroutineClosureDef , crate :: ty :: CoroutineClosureDef) ;}
mkitem!{bridge_impl ! (AliasDef , crate :: ty :: AliasDef) ;}
mkitem!{bridge_impl ! (ParamDef , crate :: ty :: ParamDef) ;}
mkitem!{bridge_impl ! (BrNamedDef , crate :: ty :: BrNamedDef) ;}
mkitem!{bridge_impl ! (TraitDef , crate :: ty :: TraitDef) ;}
mkitem!{bridge_impl ! (GenericDef , crate :: ty :: GenericDef) ;}
mkitem!{bridge_impl ! (ConstDef , crate :: ty :: ConstDef) ;}
mkitem!{bridge_impl ! (ImplDef , crate :: ty :: ImplDef) ;}
mkitem!{bridge_impl ! (RegionDef , crate :: ty :: RegionDef) ;}
mkitem!{bridge_impl ! (CoroutineWitnessDef , crate :: ty :: CoroutineWitnessDef) ;}
mkitem!{bridge_impl ! (AssocDef , crate :: ty :: AssocDef) ;}
mkitem!{bridge_impl ! (OpaqueDef , crate :: ty :: OpaqueDef) ;}
mkitem!{bridge_impl ! (StaticDef , crate :: mir :: mono :: StaticDef) ;}
mkitem!{mkimpl!{impl rustc_public_bridge :: bridge :: Prov < compiler_interface :: BridgeTys > for crate :: ty :: Prov { fn new (aid : crate :: mir :: alloc :: AllocId) -> Self { Self (aid) } }}}
mkitem!{mkimpl!{impl rustc_public_bridge :: bridge :: Allocation < compiler_interface :: BridgeTys > for crate :: ty :: Allocation { fn new < 'tcx > (bytes : Vec < Option < u8 > > , ptrs : Vec < (usize , rustc_middle :: mir :: interpret :: AllocId) > , align : u64 , mutability : rustc_middle :: mir :: Mutability , tables : & mut Tables < 'tcx , compiler_interface :: BridgeTys > , cx : & CompilerCtxt < 'tcx , compiler_interface :: BridgeTys > ,) -> Self { Self { bytes , provenance : ProvenanceMap { ptrs : ptrs . iter () . map (| (i , aid) | (* i , tables . prov (* aid))) . collect () , } , align , mutability : mutability . stable (tables , cx) , } } }}}