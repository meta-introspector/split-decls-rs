macro_rules! deps {
    () => {
        Error!();
        ContextStack!();
        Func!();
    };
}

macro_rules! gen_init_impl {
    () => {
        deps!();
        # [doc = " the init function passed to reg_context"] # [inline] pub fn gen_init_impl (_ : usize , f : * mut usize) -> ! { overflow :: init_once () ; let clo = move | | { let f : & mut Option < Func > = unsafe { & mut * (f as * mut _) } ; let func = f . take () . unwrap () ; func . call_once () ; } ; fn check_err (cause : Box < dyn Any + Send + 'static >) { if let Some (Error :: Cancel | Error :: Done) = cause . downcast_ref :: < Error > () { return ; } error ! ("set panic inside generator") ; ContextStack :: current () . top () . err = Some (cause) ; } if let Err (cause) = catch_unwind_filter (clo) { check_err (cause) ; } yield_now () ; unreachable ! ("Should never come back") ; }
    };
}

gen_init_impl!();