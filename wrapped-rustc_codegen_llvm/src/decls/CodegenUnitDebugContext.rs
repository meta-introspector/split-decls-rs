macro_rules! CodegenUnitDebugContext {
    () => {
        # [doc = " A context object for maintaining all state needed by the debuginfo module."] pub (crate) struct CodegenUnitDebugContext < 'll , 'tcx > { llmod : & 'll llvm :: Module , builder : DIBuilderBox < 'll > , created_files : RefCell < UnordMap < Option < (StableSourceFileId , SourceFileHash) > , & 'll DIFile > > , type_map : metadata :: TypeMap < 'll , 'tcx > , adt_stack : RefCell < Vec < (DefId , GenericArgsRef < 'tcx >) > > , namespace_map : RefCell < DefIdMap < & 'll DIScope > > , recursion_marker_type : OnceCell < & 'll DIType > , }
    };
}

CodegenUnitDebugContext!()