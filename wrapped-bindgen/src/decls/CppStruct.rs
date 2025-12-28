macro_rules! CppStruct {
    () => {
        # [derive (Clone , Debug)] pub struct CppStruct { pub def : TypeDef , pub name : & 'static str , pub nested : BTreeMap < & 'static str , CppStruct > , }
    };
}

CppStruct!()