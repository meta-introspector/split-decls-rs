mkuse!{use std :: time :: Instant ;}
mkuse!{use rustc_codegen_ssa :: ModuleCodegen ;}
mkuse!{use rustc_codegen_ssa :: base :: maybe_create_entry_wrapper ;}
mkuse!{use rustc_codegen_ssa :: mono_item :: MonoItemExt ;}
mkuse!{use rustc_codegen_ssa :: traits :: * ;}
mkuse!{use rustc_data_structures :: small_c_str :: SmallCStr ;}
mkuse!{use rustc_hir :: attrs :: Linkage ;}
mkuse!{use rustc_middle :: dep_graph ;}
mkuse!{use rustc_middle :: middle :: codegen_fn_attrs :: CodegenFnAttrs ;}
mkuse!{use rustc_middle :: mir :: mono :: Visibility ;}
mkuse!{use rustc_middle :: ty :: TyCtxt ;}
mkuse!{use rustc_session :: config :: DebugInfo ;}
mkuse!{use rustc_span :: Symbol ;}
mkuse!{use rustc_target :: spec :: SanitizerSet ;}
mkuse!{use super :: ModuleLlvm ;}
mkuse!{use crate :: builder :: Builder ;}
mkuse!{use crate :: context :: CodegenCx ;}
mkuse!{use crate :: value :: Value ;}
mkuse!{use crate :: { attributes , llvm } ;}
mkitem!{mkstruct!{pub (crate) struct ValueIter < 'll > { cur : Option < & 'll Value > , step : unsafe extern "C" fn (& 'll Value) -> Option < & 'll Value > , }}}
mkitem!{mkimpl!{impl < 'll > Iterator for ValueIter < 'll > { type Item = & 'll Value ; fn next (& mut self) -> Option < & 'll Value > { let old = self . cur ; if let Some (old) = old { self . cur = unsafe { (self . step) (old) } ; } old } }}}

macro_rules! iter_globals_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function iter_globals in module {}", module_path!());
    };
}

mkfn!{
    iter_globals_introspect!();
    pub (crate) fn iter_globals (llmod : & llvm :: Module) -> ValueIter < '_ > { unsafe { ValueIter { cur : llvm :: LLVMGetFirstGlobal (llmod) , step : llvm :: LLVMGetNextGlobal } } }
}

macro_rules! compile_codegen_unit_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function compile_codegen_unit in module {}", module_path!());
    };
}

mkfn!{
    compile_codegen_unit_introspect!();
    pub (crate) fn compile_codegen_unit (tcx : TyCtxt < '_ > , cgu_name : Symbol ,) -> (ModuleCodegen < ModuleLlvm > , u64) { let start_time = Instant :: now () ; let dep_node = tcx . codegen_unit (cgu_name) . codegen_dep_node (tcx) ; let (module , _) = tcx . dep_graph . with_task (dep_node , tcx , cgu_name , module_codegen , Some (dep_graph :: hash_result) ,) ; let time_to_codegen = start_time . elapsed () ; let cost = time_to_codegen . as_nanos () as u64 ; fn module_codegen (tcx : TyCtxt < '_ > , cgu_name : Symbol) -> ModuleCodegen < ModuleLlvm > { let cgu = tcx . codegen_unit (cgu_name) ; let _prof_timer = tcx . prof . generic_activity_with_arg_recorder ("codegen_module" , | recorder | { recorder . record_arg (cgu_name . to_string ()) ; recorder . record_arg (cgu . size_estimate () . to_string ()) ; }) ; let llvm_module = ModuleLlvm :: new (tcx , cgu_name . as_str ()) ; { let mut cx = CodegenCx :: new (tcx , cgu , & llvm_module) ; let mono_items = cx . codegen_unit . items_in_deterministic_order (cx . tcx) ; for & (mono_item , data) in & mono_items { mono_item . predefine :: < Builder < '_ , '_ , '_ > > (& mut cx , cgu_name . as_str () , data . linkage , data . visibility ,) ; } for & (mono_item , item_data) in & mono_items { mono_item . define :: < Builder < '_ , '_ , '_ > > (& mut cx , cgu_name . as_str () , item_data) ; } if let Some (entry) = maybe_create_entry_wrapper :: < Builder < '_ , '_ , '_ > > (& cx , cx . codegen_unit) { let attrs = attributes :: sanitize_attrs (& cx , SanitizerSet :: empty ()) ; attributes :: apply_to_llfn (entry , llvm :: AttributePlace :: Function , & attrs) ; } if cx . sess () . instrument_coverage () { cx . coverageinfo_finalize () ; } if ! cx . used_statics . is_empty () { cx . create_used_variable_impl (c"llvm.used" , & cx . used_statics) ; } if ! cx . compiler_used_statics . is_empty () { cx . create_used_variable_impl (c"llvm.compiler.used" , & cx . compiler_used_statics) ; } for & (old_g , new_g) in cx . statics_to_rauw () . borrow () . iter () { unsafe { llvm :: LLVMReplaceAllUsesWith (old_g , new_g) ; llvm :: LLVMDeleteGlobal (old_g) ; } } if cx . sess () . opts . debuginfo != DebugInfo :: None { cx . debuginfo_finalize () ; } } ModuleCodegen :: new_regular (cgu_name . to_string () , llvm_module) } (module , cost) }
}

macro_rules! set_link_section_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function set_link_section in module {}", module_path!());
    };
}

mkfn!{
    set_link_section_introspect!();
    pub (crate) fn set_link_section (llval : & Value , attrs : & CodegenFnAttrs) { let Some (sect) = attrs . link_section else { return } ; let buf = SmallCStr :: new (sect . as_str ()) ; llvm :: set_section (llval , & buf) ; }
}

macro_rules! linkage_to_llvm_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function linkage_to_llvm in module {}", module_path!());
    };
}

mkfn!{
    linkage_to_llvm_introspect!();
    pub (crate) fn linkage_to_llvm (linkage : Linkage) -> llvm :: Linkage { match linkage { Linkage :: External => llvm :: Linkage :: ExternalLinkage , Linkage :: AvailableExternally => llvm :: Linkage :: AvailableExternallyLinkage , Linkage :: LinkOnceAny => llvm :: Linkage :: LinkOnceAnyLinkage , Linkage :: LinkOnceODR => llvm :: Linkage :: LinkOnceODRLinkage , Linkage :: WeakAny => llvm :: Linkage :: WeakAnyLinkage , Linkage :: WeakODR => llvm :: Linkage :: WeakODRLinkage , Linkage :: Internal => llvm :: Linkage :: InternalLinkage , Linkage :: ExternalWeak => llvm :: Linkage :: ExternalWeakLinkage , Linkage :: Common => llvm :: Linkage :: CommonLinkage , } }
}

macro_rules! visibility_to_llvm_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function visibility_to_llvm in module {}", module_path!());
    };
}

mkfn!{
    visibility_to_llvm_introspect!();
    pub (crate) fn visibility_to_llvm (linkage : Visibility) -> llvm :: Visibility { match linkage { Visibility :: Default => llvm :: Visibility :: Default , Visibility :: Hidden => llvm :: Visibility :: Hidden , Visibility :: Protected => llvm :: Visibility :: Protected , } }
}

macro_rules! set_variable_sanitizer_attrs_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function set_variable_sanitizer_attrs in module {}", module_path!());
    };
}

mkfn!{
    set_variable_sanitizer_attrs_introspect!();
    pub (crate) fn set_variable_sanitizer_attrs (llval : & Value , attrs : & CodegenFnAttrs) { if attrs . no_sanitize . contains (SanitizerSet :: ADDRESS) { unsafe { llvm :: LLVMRustSetNoSanitizeAddress (llval) } ; } if attrs . no_sanitize . contains (SanitizerSet :: HWADDRESS) { unsafe { llvm :: LLVMRustSetNoSanitizeHWAddress (llval) } ; } }
}