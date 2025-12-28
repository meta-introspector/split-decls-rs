macro_rules! deps {
    () => {
        LocalModuleId!();
        PerNsGlobImports!();
    };
}

macro_rules! impl_60 {
    () => {
        deps!();
        impl PerNsGlobImports { pub (crate) fn contains_type (& self , module_id : LocalModuleId , name : Name) -> bool { self . types . contains (& (module_id , name)) } pub (crate) fn contains_value (& self , module_id : LocalModuleId , name : Name) -> bool { self . values . contains (& (module_id , name)) } pub (crate) fn contains_macro (& self , module_id : LocalModuleId , name : Name) -> bool { self . macros . contains (& (module_id , name)) } }
    };
}

impl_60!();