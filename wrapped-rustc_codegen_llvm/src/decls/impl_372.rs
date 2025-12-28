macro_rules! deps {
    () => {
        SCx!();
        GenericCx!();
    };
}

macro_rules! impl_372 {
    () => {
        deps!();
        impl < 'll , CX : Borrow < SCx < 'll > > > GenericCx < 'll , CX > { # [doc = " Declare a global with an intention to define it."] # [doc = ""] # [doc = " Use this function when you intend to define a global. This function will"] # [doc = " return `None` if the name already has a definition associated with it. In that"] # [doc = " case an error should be reported to the user, because it usually happens due"] # [doc = " to user’s fault (e.g., misuse of `#[no_mangle]` or `#[export_name]` attributes)."] pub (crate) fn define_global (& self , name : & str , ty : & 'll Type) -> Option < & 'll Value > { if self . get_defined_value (name) . is_some () { None } else { Some (self . declare_global (name , ty)) } } # [doc = " Declare a private global"] # [doc = ""] # [doc = " Use this function when you intend to define a global without a name."] pub (crate) fn define_private_global (& self , ty : & 'll Type) -> & 'll Value { unsafe { llvm :: LLVMRustInsertPrivateGlobal (self . llmod () , ty) } } # [doc = " Gets declared value by name."] pub (crate) fn get_declared_value (& self , name : & str) -> Option < & 'll Value > { debug ! ("get_declared_value(name={:?})" , name) ; unsafe { llvm :: LLVMRustGetNamedValue (self . llmod () , name . as_c_char_ptr () , name . len ()) } } # [doc = " Gets defined or externally defined (AvailableExternally linkage) value by"] # [doc = " name."] pub (crate) fn get_defined_value (& self , name : & str) -> Option < & 'll Value > { self . get_declared_value (name) . and_then (| val | { let declaration = llvm :: is_declaration (val) ; if ! declaration { Some (val) } else { None } }) } }
    };
}

impl_372!()