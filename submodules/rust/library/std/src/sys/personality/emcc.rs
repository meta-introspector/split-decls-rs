mkuse!{use unwind as uw ;}
mkuse!{use crate :: ffi :: c_int ;}

macro_rules! rust_eh_personality_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function rust_eh_personality in module {}", module_path!());
    };
}

mkfn!{
    rust_eh_personality_introspect!();
    # [lang = "eh_personality"] unsafe extern "C" fn rust_eh_personality (_version : c_int , _actions : uw :: _Unwind_Action , _exception_class : uw :: _Unwind_Exception_Class , _exception_object : * mut uw :: _Unwind_Exception , _context : * mut uw :: _Unwind_Context ,) -> uw :: _Unwind_Reason_Code { core :: intrinsics :: abort () }
}