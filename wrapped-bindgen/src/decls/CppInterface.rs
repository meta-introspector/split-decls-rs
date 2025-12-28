macro_rules! CppInterface {
    () => {
        # [derive (Clone , Debug , Hash , PartialEq , Eq)] pub struct CppInterface { pub def : TypeDef , }
    };
}

CppInterface!()