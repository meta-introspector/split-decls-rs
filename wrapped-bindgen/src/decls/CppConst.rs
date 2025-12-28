macro_rules! CppConst {
    () => {
        # [derive (Clone , Debug , PartialEq , Eq , Hash)] pub struct CppConst { pub namespace : & 'static str , pub field : Field , }
    };
}

CppConst!();