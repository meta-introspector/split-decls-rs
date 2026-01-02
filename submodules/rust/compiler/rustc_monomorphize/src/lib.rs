mkuse!{use rustc_hir :: lang_items :: LangItem ;}
mkuse!{use rustc_middle :: query :: TyCtxtAt ;}
mkuse!{use rustc_middle :: ty :: adjustment :: CustomCoerceUnsized ;}
mkuse!{use rustc_middle :: ty :: { self , Ty } ;}
mkuse!{use rustc_middle :: util :: Providers ;}
mkuse!{use rustc_middle :: { bug , traits } ;}
mkuse!{use rustc_span :: ErrorGuaranteed ;}
mkmod!{collector, { 
                getname!(collector);
                getsrc!(collector);
                getpath!(collector);
                get_deps!(collector);
                get_crates!(collector);
                mkinclude!(collector);
                 
            }}
mkmod!{errors, { 
                getname!(errors);
                getsrc!(errors);
                getpath!(errors);
                get_deps!(errors);
                get_crates!(errors);
                mkinclude!(errors);
                 
            }}
mkmod!{mono_checks, { 
                getname!(mono_checks);
                getsrc!(mono_checks);
                getpath!(mono_checks);
                get_deps!(mono_checks);
                get_crates!(mono_checks);
                mkinclude!(mono_checks);
                 
            }}
mkmod!{partitioning, { 
                getname!(partitioning);
                getsrc!(partitioning);
                getpath!(partitioning);
                get_deps!(partitioning);
                get_crates!(partitioning);
                mkinclude!(partitioning);
                 
            }}
mkmod!{util, { 
                getname!(util);
                getsrc!(util);
                getpath!(util);
                get_deps!(util);
                get_crates!(util);
                mkinclude!(util);
                 
            }}
mkitem!{rustc_fluent_macro :: fluent_messages ! { "messages.ftl" }}

macro_rules! custom_coerce_unsize_info_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function custom_coerce_unsize_info in module {}", module_path!());
    };
}

mkfn!{
    custom_coerce_unsize_info_introspect!();
    fn custom_coerce_unsize_info < 'tcx > (tcx : TyCtxtAt < 'tcx > , source_ty : Ty < 'tcx > , target_ty : Ty < 'tcx > ,) -> Result < CustomCoerceUnsized , ErrorGuaranteed > { let trait_ref = ty :: TraitRef :: new (tcx . tcx , tcx . require_lang_item (LangItem :: CoerceUnsized , tcx . span) , [source_ty , target_ty] ,) ; match tcx . codegen_select_candidate (ty :: TypingEnv :: fully_monomorphized () . as_query_input (trait_ref)) { Ok (traits :: ImplSource :: UserDefined (traits :: ImplSourceUserDefinedData { impl_def_id , .. })) => Ok (tcx . coerce_unsized_info (impl_def_id) ? . custom_kind . unwrap ()) , impl_source => { bug ! ("invalid `CoerceUnsized` from {source_ty} to {target_ty}: impl_source: {:?}" , impl_source) ; } } }
}

macro_rules! provide_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function provide in module {}", module_path!());
    };
}

mkfn!{
    provide_introspect!();
    pub fn provide (providers : & mut Providers) { partitioning :: provide (providers) ; mono_checks :: provide (providers) ; }
}