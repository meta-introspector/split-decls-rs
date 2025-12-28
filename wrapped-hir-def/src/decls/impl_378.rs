macro_rules! deps {
    () => {
        Enum!();
        LocalModuleId!();
        LocalDefMap!();
        ModuleId!();
        MacroId!();
        DefDatabase!();
        BuiltinShadowMode!();
        DefMap!();
        MacroSubNs!();
        PerNs!();
    };
}

macro_rules! impl_378 {
    () => {
        deps!();
        impl DefMap { pub (crate) fn block_id (& self) -> Option < BlockId > { self . block . map (| block | block . block) } pub (crate) fn prelude (& self) -> Option < (ModuleId , Option < UseId >) > { self . prelude } pub (crate) fn macro_use_prelude (& self) -> & FxHashMap < Name , (MacroId , Option < ExternCrateId >) > { & self . macro_use_prelude } pub (crate) fn resolve_path (& self , local_def_map : & LocalDefMap , db : & dyn DefDatabase , original_module : LocalModuleId , path : & ModPath , shadow : BuiltinShadowMode , expected_macro_subns : Option < MacroSubNs > ,) -> (PerNs , Option < usize >) { let res = self . resolve_path_fp_with_macro (local_def_map , db , ResolveMode :: Other , original_module , path , shadow , expected_macro_subns ,) ; (res . resolved_def , res . segment_index) } # [doc = " The first `Option<usize>` points at the `Enum` segment in case of `Enum::Variant`, the second"] # [doc = " points at the unresolved segments."] pub (crate) fn resolve_path_locally (& self , local_def_map : & LocalDefMap , db : & dyn DefDatabase , original_module : LocalModuleId , path : & ModPath , shadow : BuiltinShadowMode ,) -> (PerNs , Option < usize > , ResolvePathResultPrefixInfo) { let res = self . resolve_path_fp_with_macro_single (local_def_map , db , ResolveMode :: Other , original_module , path , shadow , None ,) ; (res . resolved_def , res . segment_index , res . prefix_info) } # [doc = " Ascends the `DefMap` hierarchy and calls `f` with every `DefMap` and containing module."] # [doc = ""] # [doc = " If `f` returns `Some(val)`, iteration is stopped and `Some(val)` is returned. If `f` returns"] # [doc = " `None`, iteration continues."] pub (crate) fn with_ancestor_maps < T > (& self , db : & dyn DefDatabase , local_mod : LocalModuleId , f : & mut dyn FnMut (& DefMap , LocalModuleId) -> Option < T > ,) -> Option < T > { if let Some (it) = f (self , local_mod) { return Some (it) ; } let mut block = self . block ; while let Some (block_info) = block { let parent = block_info . parent . def_map (db , self . krate) ; if let Some (it) = f (parent , block_info . parent . local_id) { return Some (it) ; } block = parent . block ; } None } }
    };
}

impl_378!();