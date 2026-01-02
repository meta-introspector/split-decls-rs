mkuse!{use rustc_hir :: def_id :: { DefId , LOCAL_CRATE } ;}
mkuse!{use rustc_macros :: { Decodable , Encodable , HashStable , TyDecodable , TyEncodable } ;}
mkuse!{use crate :: ty :: { self , GenericArgsRef , Ty , TyCtxt } ;}
mkitem!{mkenum!{# [doc = " The SymbolExportLevel of a symbols specifies from which kinds of crates"] # [doc = " the symbol will be exported. `C` symbols will be exported from any"] # [doc = " kind of crate, including cdylibs which export very few things."] # [doc = " `Rust` will only be exported if the crate produced is a Rust"] # [doc = " dylib."] # [derive (Eq , PartialEq , Debug , Copy , Clone , TyEncodable , TyDecodable , HashStable)] pub enum SymbolExportLevel { C , Rust , }}}
mkitem!{mkimpl!{impl SymbolExportLevel { pub fn is_below_threshold (self , threshold : SymbolExportLevel) -> bool { threshold == SymbolExportLevel :: Rust || self == SymbolExportLevel :: C } }}}
mkitem!{mkenum!{# [doc = " Kind of exported symbols."] # [derive (Eq , PartialEq , Debug , Copy , Clone , Encodable , Decodable , HashStable , Hash)] pub enum SymbolExportKind { Text , Data , Tls , }}}
mkitem!{mkstruct!{# [doc = " The `SymbolExportInfo` of a symbols specifies symbol-related information"] # [doc = " that is relevant to code generation and linking."] # [doc = ""] # [doc = " The difference between `used` and `rustc_std_internal_symbol` is that the"] # [doc = " former is exported by LTO while the latter isn't."] # [derive (Eq , PartialEq , Debug , Copy , Clone , TyEncodable , TyDecodable , HashStable)] pub struct SymbolExportInfo { pub level : SymbolExportLevel , pub kind : SymbolExportKind , # [doc = " Was the symbol marked as `#[used(compiler)]` or `#[used(linker)]`?"] pub used : bool , # [doc = " Was the symbol marked as `#[rustc_std_internal_symbol]`?"] pub rustc_std_internal_symbol : bool , }}}
mkitem!{mkenum!{# [derive (Eq , PartialEq , Debug , Copy , Clone , TyEncodable , TyDecodable , HashStable)] pub enum ExportedSymbol < 'tcx > { NonGeneric (DefId) , Generic (DefId , GenericArgsRef < 'tcx >) , DropGlue (Ty < 'tcx >) , AsyncDropGlueCtorShim (Ty < 'tcx >) , AsyncDropGlue (DefId , Ty < 'tcx >) , ThreadLocalShim (DefId) , NoDefId (ty :: SymbolName < 'tcx >) , }}}
mkitem!{mkimpl!{impl < 'tcx > ExportedSymbol < 'tcx > { # [doc = " This is the symbol name of an instance if it is instantiated in the"] # [doc = " local crate."] pub fn symbol_name_for_local_instance (& self , tcx : TyCtxt < 'tcx >) -> ty :: SymbolName < 'tcx > { match * self { ExportedSymbol :: NonGeneric (def_id) => tcx . symbol_name (ty :: Instance :: mono (tcx , def_id)) , ExportedSymbol :: Generic (def_id , args) => { tcx . symbol_name (ty :: Instance :: new_raw (def_id , args)) } ExportedSymbol :: DropGlue (ty) => { tcx . symbol_name (ty :: Instance :: resolve_drop_in_place (tcx , ty)) } ExportedSymbol :: AsyncDropGlueCtorShim (ty) => { tcx . symbol_name (ty :: Instance :: resolve_async_drop_in_place (tcx , ty)) } ExportedSymbol :: AsyncDropGlue (def_id , ty) => { tcx . symbol_name (ty :: Instance :: resolve_async_drop_in_place_poll (tcx , def_id , ty)) } ExportedSymbol :: ThreadLocalShim (def_id) => tcx . symbol_name (ty :: Instance { def : ty :: InstanceKind :: ThreadLocalShim (def_id) , args : ty :: GenericArgs :: empty () , }) , ExportedSymbol :: NoDefId (symbol_name) => symbol_name , } } }}}

macro_rules! metadata_symbol_name_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function metadata_symbol_name in module {}", module_path!());
    };
}

mkfn!{
    metadata_symbol_name_introspect!();
    pub fn metadata_symbol_name (tcx : TyCtxt < '_ >) -> String { format ! ("rust_metadata_{}_{:08x}" , tcx . crate_name (LOCAL_CRATE) , tcx . stable_crate_id (LOCAL_CRATE) ,) }
}