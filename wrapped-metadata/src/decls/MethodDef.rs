macro_rules! deps {
    () => {
        Signature!();
    };
}

macro_rules! MethodDef {
    () => {
        deps!();
        pub struct MethodDef { pub RVA : u32 , pub ImplFlags : MethodImplAttributes , pub Flags : MethodAttributes , pub Name : id :: StringId , pub Signature : id :: BlobId , pub ParamList : u32 , }
    };
}

MethodDef!();