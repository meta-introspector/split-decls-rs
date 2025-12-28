macro_rules! deps {
    () => {
        CreateAttrStringValue!();
        CodegenCx!();
        SmallVec!();
    };
}

macro_rules! patchable_function_entry_attrs {
    () => {
        deps!();
        # [inline] fn patchable_function_entry_attrs < 'll > (cx : & CodegenCx < 'll , '_ > , attr : Option < PatchableFunctionEntry > ,) -> SmallVec < [& 'll Attribute ; 2] > { let mut attrs = SmallVec :: new () ; let patchable_spec = attr . unwrap_or_else (| | { PatchableFunctionEntry :: from_config (cx . tcx . sess . opts . unstable_opts . patchable_function_entry) }) ; let entry = patchable_spec . entry () ; let prefix = patchable_spec . prefix () ; if entry > 0 { attrs . push (llvm :: CreateAttrStringValue (cx . llcx , "patchable-function-entry" , & format ! ("{}" , entry) ,)) ; } if prefix > 0 { attrs . push (llvm :: CreateAttrStringValue (cx . llcx , "patchable-function-prefix" , & format ! ("{}" , prefix) ,)) ; } attrs }
    };
}

patchable_function_entry_attrs!()