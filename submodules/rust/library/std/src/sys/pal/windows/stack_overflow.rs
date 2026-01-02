mkuse!{use crate :: sys :: c ;}
mkuse!{use crate :: thread ;}

macro_rules! reserve_stack_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function reserve_stack in module {}", module_path!());
    };
}

mkfn!{
    reserve_stack_introspect!();
    # [doc = " Reserve stack space for use in stack overflow exceptions."] pub fn reserve_stack () { let result = unsafe { c :: SetThreadStackGuarantee (& mut 0x5000) } ; debug_assert_ne ! (result , 0 , "failed to reserve stack space for exception handling") ; }
}

macro_rules! vectored_handler_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function vectored_handler in module {}", module_path!());
    };
}

mkfn!{
    vectored_handler_introspect!();
    unsafe extern "system" fn vectored_handler (ExceptionInfo : * mut c :: EXCEPTION_POINTERS) -> i32 { unsafe { let rec = & (* (* ExceptionInfo) . ExceptionRecord) ; let code = rec . ExceptionCode ; if code == c :: EXCEPTION_STACK_OVERFLOW { thread :: with_current_name (| name | { let name = name . unwrap_or ("<unknown>") ; let tid = thread :: current_os_id () ; rtprintpanic ! ("\nthread '{name}' ({tid}) has overflowed its stack\n") ; }) ; } c :: EXCEPTION_CONTINUE_SEARCH } }
}

macro_rules! init_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function init in module {}", module_path!());
    };
}

mkfn!{
    init_introspect!();
    pub fn init () { unsafe { let result = c :: AddVectoredExceptionHandler (0 , Some (vectored_handler)) ; debug_assert ! (! result . is_null () , "failed to install exception handler") ; } reserve_stack () ; }
}