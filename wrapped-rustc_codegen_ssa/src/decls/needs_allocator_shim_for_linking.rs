macro_rules! needs_allocator_shim_for_linking {
    () => {
        # [doc = " Decide if this particular crate type needs an allocator shim linked in."] # [doc = " This may return true even when allocator_kind_for_codegen returns false. In"] # [doc = " this case no allocator shim shall be linked."] pub (crate) fn needs_allocator_shim_for_linking (dependency_formats : & Dependencies , crate_type : CrateType ,) -> bool { use rustc_middle :: middle :: dependency_format :: Linkage ; let any_dynamic_crate = dependency_formats [& crate_type] . iter () . any (| & linkage | linkage == Linkage :: Dynamic) ; ! any_dynamic_crate }
    };
}

needs_allocator_shim_for_linking!();