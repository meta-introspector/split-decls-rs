macro_rules! deps {
    () => {
        VTab!();
        Name!();
        Error!();
        Module!();
        Result!();
        InnerConnection!();
    };
}

macro_rules! impl_586 {
    () => {
        deps!();
        impl InnerConnection { fn create_module < 'vtab , T : VTab < 'vtab > , M : Name > (& mut self , module_name : M , module : & 'static Module < 'vtab , T > , aux : Option < T :: Aux > ,) -> Result < () > { use crate :: version ; if version :: version_number () < 3_009_000 && module . base . xCreate . is_none () { return Err (Error :: ModuleError (format ! ("Eponymous-only virtual table not supported by SQLite version {}" , version :: version ()))) ; } let c_name = module_name . as_cstr () ? ; let r = match aux { Some (aux) => { let boxed_aux : * mut T :: Aux = Box :: into_raw (Box :: new (aux)) ; unsafe { ffi :: sqlite3_create_module_v2 (self . db () , c_name . as_ptr () , & module . base , boxed_aux . cast :: < c_void > () , Some (free_boxed_value :: < T :: Aux >) ,) } } None => unsafe { ffi :: sqlite3_create_module_v2 (self . db () , c_name . as_ptr () , & module . base , ptr :: null_mut () , None ,) } , } ; self . decode_result (r) } }
    };
}

impl_586!()