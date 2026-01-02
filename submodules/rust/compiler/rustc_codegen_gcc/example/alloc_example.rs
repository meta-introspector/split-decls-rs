mkitem!{extern crate alloc ;}
mkitem!{extern crate alloc_system ;}
mkuse!{use alloc :: boxed :: Box ;}
mkuse!{use alloc_system :: System ;}
mkitem!{# [global_allocator] static ALLOC : System = System ;}
mkitem!{# [link (name = "c")] extern "C" { fn puts (s : * const u8) -> i32 ; }}

macro_rules! panic_handler_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function panic_handler in module {}", module_path!());
    };
}

mkfn!{
    panic_handler_introspect!();
    # [panic_handler] fn panic_handler (_ : & core :: panic :: PanicInfo < '_ >) -> ! { core :: intrinsics :: abort () ; }
}

macro_rules! alloc_error_handler_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function alloc_error_handler in module {}", module_path!());
    };
}

mkfn!{
    alloc_error_handler_introspect!();
    # [alloc_error_handler] fn alloc_error_handler (_ : alloc :: alloc :: Layout) -> ! { core :: intrinsics :: abort () ; }
}

macro_rules! eh_personality_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function eh_personality in module {}", module_path!());
    };
}

mkfn!{
    eh_personality_introspect!();
    # [lang = "eh_personality"] fn eh_personality () -> ! { loop { } }
}

macro_rules! _Unwind_Resume_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function _Unwind_Resume in module {}", module_path!());
    };
}

mkfn!{
    _Unwind_Resume_introspect!();
    # [no_mangle] unsafe extern "C" fn _Unwind_Resume () { core :: intrinsics :: unreachable () ; }
}

macro_rules! main_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function main in module {}", module_path!());
    };
}

mkfn!{
    main_introspect!();
    # [no_mangle] extern "C" fn main (_argc : core :: ffi :: c_int , _argv : * const * const u8) -> core :: ffi :: c_int { let world : Box < & str > = Box :: new ("Hello World!\0") ; unsafe { puts (* world as * const str as * const u8) ; } 0 }
}