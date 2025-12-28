macro_rules! deps {
    () => {
        BackendTypes!();
    };
}

macro_rules! TypeMembershipCodegenMethods {
    () => {
        deps!();
        pub trait TypeMembershipCodegenMethods < 'tcx > : BackendTypes { fn add_type_metadata (& self , _function : Self :: Function , _typeid : & [u8]) { } fn set_type_metadata (& self , _function : Self :: Function , _typeid : & [u8]) { } fn typeid_metadata (& self , _typeid : & [u8]) -> Option < Self :: Metadata > { None } fn add_kcfi_type_metadata (& self , _function : Self :: Function , _typeid : u32) { } fn set_kcfi_type_metadata (& self , _function : Self :: Function , _typeid : u32) { } }
    };
}

TypeMembershipCodegenMethods!();