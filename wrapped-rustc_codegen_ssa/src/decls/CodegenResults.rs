macro_rules! deps {
    () => {
        CompiledModule!();
        CrateInfo!();
    };
}

macro_rules! CodegenResults {
    () => {
        deps!();
        # [derive (Encodable , Decodable)] pub struct CodegenResults { pub modules : Vec < CompiledModule > , pub allocator_module : Option < CompiledModule > , pub crate_info : CrateInfo , }
    };
}

CodegenResults!();