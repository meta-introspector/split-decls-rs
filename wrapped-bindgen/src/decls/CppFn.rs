macro_rules! CppFn {
    () => {
        # [derive (Clone , Debug , PartialEq , Eq , Hash)] pub struct CppFn { pub namespace : & 'static str , pub method : MethodDef , }
    };
}

CppFn!()