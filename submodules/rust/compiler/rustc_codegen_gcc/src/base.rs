mkuse!{use std :: collections :: HashSet ;}
mkuse!{use std :: env ;}
mkuse!{use std :: sync :: Arc ;}
mkuse!{use std :: time :: Instant ;}
mkuse!{use gccjit :: { CType , Context , FunctionType , GlobalKind } ;}
mkuse!{use rustc_codegen_ssa :: ModuleCodegen ;}
mkuse!{use rustc_codegen_ssa :: base :: maybe_create_entry_wrapper ;}
mkuse!{use rustc_codegen_ssa :: mono_item :: MonoItemExt ;}
mkuse!{use rustc_codegen_ssa :: traits :: DebugInfoCodegenMethods ;}
mkuse!{use rustc_hir :: attrs :: Linkage ;}
mkuse!{use rustc_middle :: dep_graph ;}
mkuse!{# [cfg (feature = "master")] use rustc_middle :: mir :: mono :: Visibility ;}
mkuse!{use rustc_middle :: ty :: TyCtxt ;}
mkuse!{use rustc_session :: config :: DebugInfo ;}
mkuse!{use rustc_span :: Symbol ;}
mkuse!{# [cfg (feature = "master")] use rustc_target :: spec :: SymbolVisibility ;}
mkuse!{use rustc_target :: spec :: { PanicStrategy , RelocModel } ;}
mkuse!{use crate :: builder :: Builder ;}
mkuse!{use crate :: context :: CodegenCx ;}
mkuse!{use crate :: { GccContext , LockedTargetInfo , SyncContext , gcc_util , new_context } ;}

macro_rules! visibility_to_gcc_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function visibility_to_gcc in module {}", module_path!());
    };
}

mkfn!{
    visibility_to_gcc_introspect!();
    # [cfg (feature = "master")] pub fn visibility_to_gcc (visibility : Visibility) -> gccjit :: Visibility { match visibility { Visibility :: Default => gccjit :: Visibility :: Default , Visibility :: Hidden => gccjit :: Visibility :: Hidden , Visibility :: Protected => gccjit :: Visibility :: Protected , } }
}

macro_rules! symbol_visibility_to_gcc_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function symbol_visibility_to_gcc in module {}", module_path!());
    };
}

mkfn!{
    symbol_visibility_to_gcc_introspect!();
    # [cfg (feature = "master")] pub fn symbol_visibility_to_gcc (visibility : SymbolVisibility) -> gccjit :: Visibility { match visibility { SymbolVisibility :: Hidden => gccjit :: Visibility :: Hidden , SymbolVisibility :: Protected => gccjit :: Visibility :: Protected , SymbolVisibility :: Interposable => gccjit :: Visibility :: Default , } }
}

macro_rules! global_linkage_to_gcc_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function global_linkage_to_gcc in module {}", module_path!());
    };
}

mkfn!{
    global_linkage_to_gcc_introspect!();
    pub fn global_linkage_to_gcc (linkage : Linkage) -> GlobalKind { match linkage { Linkage :: External => GlobalKind :: Imported , Linkage :: AvailableExternally => GlobalKind :: Imported , Linkage :: LinkOnceAny => unimplemented ! () , Linkage :: LinkOnceODR => unimplemented ! () , Linkage :: WeakAny => unimplemented ! () , Linkage :: WeakODR => unimplemented ! () , Linkage :: Internal => GlobalKind :: Internal , Linkage :: ExternalWeak => GlobalKind :: Imported , Linkage :: Common => unimplemented ! () , } }
}

macro_rules! linkage_to_gcc_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function linkage_to_gcc in module {}", module_path!());
    };
}

mkfn!{
    linkage_to_gcc_introspect!();
    pub fn linkage_to_gcc (linkage : Linkage) -> FunctionType { match linkage { Linkage :: External => FunctionType :: Exported , Linkage :: AvailableExternally => FunctionType :: Extern , Linkage :: LinkOnceAny => unimplemented ! () , Linkage :: LinkOnceODR => unimplemented ! () , Linkage :: WeakAny => FunctionType :: Exported , Linkage :: WeakODR => unimplemented ! () , Linkage :: Internal => FunctionType :: Internal , Linkage :: ExternalWeak => unimplemented ! () , Linkage :: Common => unimplemented ! () , } }
}

macro_rules! compile_codegen_unit_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function compile_codegen_unit in module {}", module_path!());
    };
}

mkfn!{
    compile_codegen_unit_introspect!();
    pub fn compile_codegen_unit (tcx : TyCtxt < '_ > , cgu_name : Symbol , target_info : LockedTargetInfo ,) -> (ModuleCodegen < GccContext > , u64) { let prof_timer = tcx . prof . generic_activity ("codegen_module") ; let start_time = Instant :: now () ; let dep_node = tcx . codegen_unit (cgu_name) . codegen_dep_node (tcx) ; let (module , _) = tcx . dep_graph . with_task (dep_node , tcx , (cgu_name , target_info) , module_codegen , Some (dep_graph :: hash_result) ,) ; let time_to_codegen = start_time . elapsed () ; drop (prof_timer) ; let cost = time_to_codegen . as_secs () * 1_000_000_000 + time_to_codegen . subsec_nanos () as u64 ; fn module_codegen (tcx : TyCtxt < '_ > , (cgu_name , target_info) : (Symbol , LockedTargetInfo) ,) -> ModuleCodegen < GccContext > { let cgu = tcx . codegen_unit (cgu_name) ; let context = new_context (tcx) ; if tcx . sess . panic_strategy () == PanicStrategy :: Unwind { context . add_command_line_option ("-fexceptions") ; context . add_driver_option ("-fexceptions") ; } let disabled_features : HashSet < _ > = tcx . sess . opts . cg . target_feature . split (',') . filter (| feature | feature . starts_with ('-')) . map (| string | & string [1 ..]) . collect () ; if ! disabled_features . contains ("avx") && tcx . sess . target . arch == "x86_64" { context . add_command_line_option ("-mavx") ; } for arg in & tcx . sess . opts . cg . llvm_args { context . add_command_line_option (arg) ; } context . add_command_line_option ("-fno-var-tracking-assignments") ; context . add_command_line_option ("-fno-semantic-interposition") ; context . add_command_line_option ("-fno-strict-aliasing") ; context . add_command_line_option ("-fwrapv") ; if let Some (model) = tcx . sess . code_model () { use rustc_target :: spec :: CodeModel ; context . add_command_line_option (match model { CodeModel :: Tiny => "-mcmodel=tiny" , CodeModel :: Small => "-mcmodel=small" , CodeModel :: Kernel => "-mcmodel=kernel" , CodeModel :: Medium => "-mcmodel=medium" , CodeModel :: Large => "-mcmodel=large" , }) ; } add_pic_option (& context , tcx . sess . relocation_model ()) ; let target_cpu = gcc_util :: target_cpu (tcx . sess) ; if target_cpu != "generic" { context . add_command_line_option (format ! ("-march={}" , target_cpu)) ; } if tcx . sess . opts . unstable_opts . function_sections . unwrap_or (tcx . sess . target . function_sections) { context . add_command_line_option ("-ffunction-sections") ; context . add_command_line_option ("-fdata-sections") ; } if env :: var ("CG_GCCJIT_DUMP_RTL") . as_deref () == Ok ("1") { context . add_command_line_option ("-fdump-rtl-vregs") ; } if env :: var ("CG_GCCJIT_DUMP_RTL_ALL") . as_deref () == Ok ("1") { context . add_command_line_option ("-fdump-rtl-all") ; } if env :: var ("CG_GCCJIT_DUMP_TREE_ALL") . as_deref () == Ok ("1") { context . add_command_line_option ("-fdump-tree-all-eh") ; } if env :: var ("CG_GCCJIT_DUMP_IPA_ALL") . as_deref () == Ok ("1") { context . add_command_line_option ("-fdump-ipa-all-eh") ; } if env :: var ("CG_GCCJIT_DUMP_CODE") . as_deref () == Ok ("1") { context . set_dump_code_on_compile (true) ; } if env :: var ("CG_GCCJIT_DUMP_GIMPLE") . as_deref () == Ok ("1") { context . set_dump_initial_gimple (true) ; } if env :: var ("CG_GCCJIT_DUMP_EVERYTHING") . as_deref () == Ok ("1") { context . set_dump_everything (true) ; } if env :: var ("CG_GCCJIT_KEEP_INTERMEDIATES") . as_deref () == Ok ("1") { context . set_keep_intermediates (true) ; } if env :: var ("CG_GCCJIT_VERBOSE") . as_deref () == Ok ("1") { context . add_driver_option ("-v") ; } context . set_allow_unreachable_blocks (true) ; { let f16_type_supported = target_info . supports_target_dependent_type (CType :: Float16) ; let f32_type_supported = target_info . supports_target_dependent_type (CType :: Float32) ; let f64_type_supported = target_info . supports_target_dependent_type (CType :: Float64) ; let f128_type_supported = target_info . supports_target_dependent_type (CType :: Float128) ; let u128_type_supported = target_info . supports_target_dependent_type (CType :: UInt128t) ; let mut cx = CodegenCx :: new (& context , cgu , tcx , u128_type_supported , f16_type_supported , f32_type_supported , f64_type_supported , f128_type_supported ,) ; let mono_items = cgu . items_in_deterministic_order (tcx) ; for & (mono_item , data) in & mono_items { mono_item . predefine :: < Builder < '_ , '_ , '_ > > (& mut cx , cgu_name . as_str () , data . linkage , data . visibility ,) ; } for & (mono_item , item_data) in & mono_items { mono_item . define :: < Builder < '_ , '_ , '_ > > (& mut cx , cgu_name . as_str () , item_data) ; } maybe_create_entry_wrapper :: < Builder < '_ , '_ , '_ > > (& cx , cx . codegen_unit) ; if cx . sess () . opts . debuginfo != DebugInfo :: None { cx . debuginfo_finalize () ; } } ModuleCodegen :: new_regular (cgu_name . to_string () , GccContext { context : Arc :: new (SyncContext :: new (context)) , relocation_model : tcx . sess . relocation_model () , should_combine_object_files : false , temp_dir : None , } ,) } (module , cost) }
}

macro_rules! add_pic_option_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function add_pic_option in module {}", module_path!());
    };
}

mkfn!{
    add_pic_option_introspect!();
    pub fn add_pic_option < 'gcc > (context : & Context < 'gcc > , relocation_model : RelocModel) { match relocation_model { rustc_target :: spec :: RelocModel :: Static => { context . add_command_line_option ("-fno-pie") ; context . add_driver_option ("-fno-pie") ; } rustc_target :: spec :: RelocModel :: Pic => { context . add_command_line_option ("-fPIC") ; context . add_driver_option ("-fPIC") ; } rustc_target :: spec :: RelocModel :: Pie => { context . add_command_line_option ("-fPIE") ; context . add_driver_option ("-fPIE") ; } model => eprintln ! ("Unsupported relocation model: {:?}" , model) , } }
}