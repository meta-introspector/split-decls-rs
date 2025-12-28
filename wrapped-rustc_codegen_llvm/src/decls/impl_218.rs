macro_rules! deps {
    () => {
        CodegenCx!();
        SimpleCx!();
        Linkage!();
        CodegenUnitDebugContext!();
        FullCx!();
        CguCoverageContext!();
        ModuleLlvm!();
        GenericCx!();
    };
}

macro_rules! impl_218 {
    () => {
        deps!();
        impl < 'll , 'tcx > CodegenCx < 'll , 'tcx > { pub (crate) fn new (tcx : TyCtxt < 'tcx > , codegen_unit : & 'tcx CodegenUnit < 'tcx > , llvm_module : & 'll crate :: ModuleLlvm ,) -> Self { let use_dll_storage_attrs = tcx . sess . target . is_like_windows ; let tls_model = to_llvm_tls_model (tcx . sess . tls_model ()) ; let (llcx , llmod) = (& * llvm_module . llcx , llvm_module . llmod ()) ; let coverage_cx = tcx . sess . instrument_coverage () . then (coverageinfo :: CguCoverageContext :: new) ; let dbg_cx = if tcx . sess . opts . debuginfo != DebugInfo :: None { let dctx = debuginfo :: CodegenUnitDebugContext :: new (llmod) ; debuginfo :: metadata :: build_compile_unit_di_node (tcx , codegen_unit . name () . as_str () , & dctx ,) ; Some (dctx) } else { None } ; GenericCx (FullCx { tcx , scx : SimpleCx :: new (llmod , llcx , tcx . data_layout . pointer_size ()) , use_dll_storage_attrs , tls_model , codegen_unit , instances : Default :: default () , vtables : Default :: default () , const_str_cache : Default :: default () , const_globals : Default :: default () , statics_to_rauw : RefCell :: new (Vec :: new ()) , used_statics : Vec :: new () , compiler_used_statics : Vec :: new () , type_lowering : Default :: default () , scalar_lltypes : Default :: default () , coverage_cx , dbg_cx , eh_personality : Cell :: new (None) , eh_catch_typeinfo : Cell :: new (None) , rust_try_fn : Cell :: new (None) , intrinsics : Default :: default () , local_gen_sym_counter : Cell :: new (0) , renamed_statics : Default :: default () , } , PhantomData ,) } pub (crate) fn statics_to_rauw (& self) -> & RefCell < Vec < (& 'll Value , & 'll Value) > > { & self . statics_to_rauw } # [doc = " Extra state that is only available when coverage instrumentation is enabled."] # [inline] # [track_caller] pub (crate) fn coverage_cx (& self) -> & coverageinfo :: CguCoverageContext < 'll , 'tcx > { self . coverage_cx . as_ref () . expect ("only called when coverage instrumentation is enabled") } pub (crate) fn create_used_variable_impl (& self , name : & 'static CStr , values : & [& 'll Value]) { let array = self . const_array (self . type_ptr () , values) ; let g = llvm :: add_global (self . llmod , self . val_ty (array) , name) ; llvm :: set_initializer (g , array) ; llvm :: set_linkage (g , llvm :: Linkage :: AppendingLinkage) ; llvm :: set_section (g , c"llvm.metadata") ; } }
    };
}

impl_218!()