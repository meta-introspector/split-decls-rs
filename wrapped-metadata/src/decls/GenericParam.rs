macro_rules! GenericParam {
    () => {
        # [derive (Copy , Clone)] pub struct GenericParam { pub Number : u16 , pub Flags : GenericParamAttributes , pub Owner : TypeOrMethodDef , pub Name : id :: StringId , }
    };
}

GenericParam!();