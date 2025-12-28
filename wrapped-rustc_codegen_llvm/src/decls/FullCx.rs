macro_rules! deps {
    () => {
        SimpleCx!();
        CguCoverageContext!();
        SmallVec!();
        CodegenUnitDebugContext!();
        ThreadLocalMode!();
    };
}

macro_rules! FullCx {
    () => {
        deps!();
        pub (crate) struct FullCx < 'll , 'tcx > { pub tcx : TyCtxt < 'tcx > , pub scx : SimpleCx < 'll > , pub use_dll_storage_attrs : bool , pub tls_model : llvm :: ThreadLocalMode , pub codegen_unit : & 'tcx CodegenUnit < 'tcx > , # [doc = " Cache instances of monomorphic and polymorphic items"] pub instances : RefCell < FxHashMap < Instance < 'tcx > , & 'll Value > > , # [doc = " Cache generated vtables"] pub vtables : RefCell < FxHashMap < (Ty < 'tcx > , Option < ty :: ExistentialTraitRef < 'tcx > >) , & 'll Value > > , # [doc = " Cache of constant strings,"] pub const_str_cache : RefCell < FxHashMap < String , & 'll Value > > , # [doc = " Cache of emitted const globals (value -> global)"] pub const_globals : RefCell < FxHashMap < & 'll Value , & 'll Value > > , # [doc = " List of globals for static variables which need to be passed to the"] # [doc = " LLVM function ReplaceAllUsesWith (RAUW) when codegen is complete."] # [doc = " (We have to make sure we don't invalidate any Values referring"] # [doc = " to constants.)"] pub statics_to_rauw : RefCell < Vec < (& 'll Value , & 'll Value) > > , # [doc = " Statics that will be placed in the llvm.used variable"] # [doc = " See <https://llvm.org/docs/LangRef.html#the-llvm-used-global-variable> for details"] pub used_statics : Vec < & 'll Value > , # [doc = " Statics that will be placed in the llvm.compiler.used variable"] # [doc = " See <https://llvm.org/docs/LangRef.html#the-llvm-compiler-used-global-variable> for details"] pub compiler_used_statics : Vec < & 'll Value > , # [doc = " Mapping of non-scalar types to llvm types."] pub type_lowering : RefCell < FxHashMap < (Ty < 'tcx > , Option < VariantIdx >) , & 'll Type > > , # [doc = " Mapping of scalar types to llvm types."] pub scalar_lltypes : RefCell < FxHashMap < Ty < 'tcx > , & 'll Type > > , # [doc = " Extra per-CGU codegen state needed when coverage instrumentation is enabled."] pub coverage_cx : Option < coverageinfo :: CguCoverageContext < 'll , 'tcx > > , pub dbg_cx : Option < debuginfo :: CodegenUnitDebugContext < 'll , 'tcx > > , eh_personality : Cell < Option < & 'll Value > > , eh_catch_typeinfo : Cell < Option < & 'll Value > > , pub rust_try_fn : Cell < Option < (& 'll Type , & 'll Value) > > , intrinsics : RefCell < FxHashMap < (Cow < 'static , str > , SmallVec < [& 'll Type ; 2] >) , (& 'll Type , & 'll Value) > > , # [doc = " A counter that is used for generating local symbol names"] local_gen_sym_counter : Cell < usize > , # [doc = " `codegen_static` will sometimes create a second global variable with a"] # [doc = " different type and clear the symbol name of the original global."] # [doc = " `global_asm!` needs to be able to find this new global so that it can"] # [doc = " compute the correct mangled symbol name to insert into the asm."] pub renamed_statics : RefCell < FxHashMap < DefId , & 'll Value > > , }
    };
}

FullCx!()