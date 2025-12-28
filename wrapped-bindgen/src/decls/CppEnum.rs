macro_rules! CppEnum {
    () => {
        # [derive (Clone , Debug , PartialEq , Eq , Hash)] pub struct CppEnum { pub def : TypeDef , }
    };
}

CppEnum!()