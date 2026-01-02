mkuse!{use rustc_hir :: def :: DefKind ;}
mkuse!{use rustc_hir :: def_id :: { CrateNum , LOCAL_CRATE } ;}
mkuse!{use rustc_middle :: middle :: codegen_fn_attrs :: { CodegenFnAttrFlags , CodegenFnAttrs } ;}
mkuse!{use rustc_middle :: mir :: mono :: { InstantiationMode , MonoItem } ;}
mkuse!{use rustc_middle :: query :: Providers ;}
mkuse!{use rustc_middle :: ty :: { self , Instance , TyCtxt } ;}
mkuse!{use rustc_session :: config :: SymbolManglingVersion ;}
mkuse!{use tracing :: debug ;}
mkmod!{export, { 
                getname!(export);
                getsrc!(export);
                getpath!(export);
                get_deps!(export);
                get_crates!(export);
                mkinclude!(export);
                 
            }}
mkmod!{hashed, { 
                getname!(hashed);
                getsrc!(hashed);
                getpath!(hashed);
                get_deps!(hashed);
                get_crates!(hashed);
                mkinclude!(hashed);
                 
            }}
mkmod!{legacy, { 
                getname!(legacy);
                getsrc!(legacy);
                getpath!(legacy);
                get_deps!(legacy);
                get_crates!(legacy);
                mkinclude!(legacy);
                 
            }}
mkmod!{v0, { 
                getname!(v0);
                getsrc!(v0);
                getpath!(v0);
                get_deps!(v0);
                get_crates!(v0);
                mkinclude!(v0);
                 
            }}
mkmod!{errors, { 
                getname!(errors);
                getsrc!(errors);
                getpath!(errors);
                get_deps!(errors);
                get_crates!(errors);
                mkinclude!(errors);
                 
            }}
mkmod!{test, { 
                getname!(test);
                getsrc!(test);
                getpath!(test);
                get_deps!(test);
                get_crates!(test);
                mkinclude!(test);
                 
            }}
mkuse!{pub use v0 :: mangle_internal_symbol ;}

macro_rules! symbol_name_for_instance_in_crate_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function symbol_name_for_instance_in_crate in module {}", module_path!());
    };
}

mkfn!{
    symbol_name_for_instance_in_crate_introspect!();
    # [doc = " This function computes the symbol name for the given `instance` and the"] # [doc = " given instantiating crate. That is, if you know that instance X is"] # [doc = " instantiated in crate Y, this is the symbol name this instance would have."] pub fn symbol_name_for_instance_in_crate < 'tcx > (tcx : TyCtxt < 'tcx > , instance : Instance < 'tcx > , instantiating_crate : CrateNum ,) -> String { compute_symbol_name (tcx , instance , | | instantiating_crate) }
}

macro_rules! provide_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function provide in module {}", module_path!());
    };
}

mkfn!{
    provide_introspect!();
    pub fn provide (providers : & mut Providers) { * providers = Providers { symbol_name : symbol_name_provider , .. * providers } ; }
}

macro_rules! symbol_name_provider_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function symbol_name_provider in module {}", module_path!());
    };
}

mkfn!{
    symbol_name_provider_introspect!();
    fn symbol_name_provider < 'tcx > (tcx : TyCtxt < 'tcx > , instance : Instance < 'tcx >) -> ty :: SymbolName < 'tcx > { let symbol_name = compute_symbol_name (tcx , instance , | | { if is_generic (instance) { instance . upstream_monomorphization (tcx) . unwrap_or (LOCAL_CRATE) } else { LOCAL_CRATE } }) ; ty :: SymbolName :: new (tcx , & symbol_name) }
}

macro_rules! typeid_for_trait_ref_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function typeid_for_trait_ref in module {}", module_path!());
    };
}

mkfn!{
    typeid_for_trait_ref_introspect!();
    pub fn typeid_for_trait_ref < 'tcx > (tcx : TyCtxt < 'tcx > , trait_ref : ty :: ExistentialTraitRef < 'tcx > ,) -> String { v0 :: mangle_typeid_for_trait_ref (tcx , trait_ref) }
}

macro_rules! compute_symbol_name_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function compute_symbol_name in module {}", module_path!());
    };
}

mkfn!{
    compute_symbol_name_introspect!();
    # [doc = " Computes the symbol name for the given instance. This function will call"] # [doc = " `compute_instantiating_crate` if it needs to factor the instantiating crate"] # [doc = " into the symbol name."] fn compute_symbol_name < 'tcx > (tcx : TyCtxt < 'tcx > , instance : Instance < 'tcx > , compute_instantiating_crate : impl FnOnce () -> CrateNum ,) -> String { let def_id = instance . def_id () ; let args = instance . args ; debug ! ("symbol_name(def_id={:?}, args={:?})" , def_id , args) ; if let Some (def_id) = def_id . as_local () { if tcx . proc_macro_decls_static (()) == Some (def_id) { let stable_crate_id = tcx . stable_crate_id (LOCAL_CRATE) ; return tcx . sess . generate_proc_macro_decls_symbol (stable_crate_id) ; } } let attrs = if tcx . def_kind (def_id) . has_codegen_attrs () { & tcx . codegen_instance_attrs (instance . def) } else { CodegenFnAttrs :: EMPTY } ; if attrs . flags . contains (CodegenFnAttrFlags :: RUSTC_STD_INTERNAL_SYMBOL) { let name = if let Some (name) = attrs . symbol_name { name } else { tcx . item_name (def_id) } ; return v0 :: mangle_internal_symbol (tcx , name . as_str ()) ; } let wasm_import_module_exception_force_mangling = { tcx . is_foreign_item (def_id) && tcx . sess . target . is_like_wasm && tcx . wasm_import_module_map (def_id . krate) . contains_key (& def_id . into ()) } ; if ! wasm_import_module_exception_force_mangling { if let Some (name) = attrs . symbol_name { return name . to_string () ; } if attrs . flags . contains (CodegenFnAttrFlags :: NO_MANGLE) { return tcx . item_name (def_id) . to_string () ; } } let is_globally_shared_function = matches ! (tcx . def_kind (instance . def_id ()) , DefKind :: Fn | DefKind :: AssocFn | DefKind :: Closure | DefKind :: SyntheticCoroutineBody | DefKind :: Ctor (..)) && matches ! (MonoItem :: Fn (instance) . instantiation_mode (tcx) , InstantiationMode :: GloballyShared { may_conflict : true }) ; let avoid_cross_crate_conflicts = is_generic (instance) || is_globally_shared_function ; let instantiating_crate = avoid_cross_crate_conflicts . then (compute_instantiating_crate) ; let mangling_version_crate = instantiating_crate . unwrap_or (def_id . krate) ; let mangling_version = if mangling_version_crate == LOCAL_CRATE { tcx . sess . opts . get_symbol_mangling_version () } else { tcx . symbol_mangling_version (mangling_version_crate) } ; let symbol = match tcx . is_exportable (def_id) { true => format ! ("{}.{}" , v0 :: mangle (tcx , instance , instantiating_crate , true) , export :: compute_hash_of_export_fn (tcx , instance)) , false => match mangling_version { SymbolManglingVersion :: Legacy => legacy :: mangle (tcx , instance , instantiating_crate) , SymbolManglingVersion :: V0 => v0 :: mangle (tcx , instance , instantiating_crate , false) , SymbolManglingVersion :: Hashed => { hashed :: mangle (tcx , instance , instantiating_crate , | | { v0 :: mangle (tcx , instance , instantiating_crate , false) }) } } , } ; debug_assert ! (rustc_demangle :: try_demangle (& symbol) . is_ok () , "compute_symbol_name: `{symbol}` cannot be demangled") ; symbol }
}

macro_rules! is_generic_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function is_generic in module {}", module_path!());
    };
}

mkfn!{
    is_generic_introspect!();
    fn is_generic < 'tcx > (instance : Instance < 'tcx >) -> bool { instance . args . non_erasable_generics () . next () . is_some () }
}