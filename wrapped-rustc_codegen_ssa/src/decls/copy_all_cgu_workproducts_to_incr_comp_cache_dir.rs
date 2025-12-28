macro_rules! deps {
    () => {
        ModuleKind!();
        CompiledModules!();
    };
}

macro_rules! copy_all_cgu_workproducts_to_incr_comp_cache_dir {
    () => {
        deps!();
        fn copy_all_cgu_workproducts_to_incr_comp_cache_dir (sess : & Session , compiled_modules : & CompiledModules ,) -> FxIndexMap < WorkProductId , WorkProduct > { let mut work_products = FxIndexMap :: default () ; if sess . opts . incremental . is_none () { return work_products ; } let _timer = sess . timer ("copy_all_cgu_workproducts_to_incr_comp_cache_dir") ; for module in compiled_modules . modules . iter () . filter (| m | m . kind == ModuleKind :: Regular) { let mut files = Vec :: new () ; if let Some (object_file_path) = & module . object { files . push ((OutputType :: Object . extension () , object_file_path . as_path ())) ; } if let Some (dwarf_object_file_path) = & module . dwarf_object { files . push (("dwo" , dwarf_object_file_path . as_path ())) ; } if let Some (path) = & module . assembly { files . push ((OutputType :: Assembly . extension () , path . as_path ())) ; } if let Some (path) = & module . llvm_ir { files . push ((OutputType :: LlvmAssembly . extension () , path . as_path ())) ; } if let Some (path) = & module . bytecode { files . push ((OutputType :: Bitcode . extension () , path . as_path ())) ; } if let Some ((id , product)) = copy_cgu_workproduct_to_incr_comp_cache_dir (sess , & module . name , files . as_slice () , & module . links_from_incr_cache ,) { work_products . insert (id , product) ; } } work_products }
    };
}

copy_all_cgu_workproducts_to_incr_comp_cache_dir!()