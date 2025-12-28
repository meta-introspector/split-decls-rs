macro_rules! deps {
    () => {
        BindIndex!();
        Error!();
        Statement!();
        Result!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        # [doc = " C-string literal to avoid alloc"] impl BindIndex for & CStr { fn idx (& self , stmt : & Statement < '_ >) -> Result < usize > { let r = unsafe { ffi :: sqlite3_bind_parameter_index (stmt . ptr () , self . as_ptr ()) } ; match r { 0 => Err (Error :: InvalidParameterName (self . to_string_lossy () . to_string () ,)) , i => Ok (i as usize) , } } }
    };
}

impl_39!()