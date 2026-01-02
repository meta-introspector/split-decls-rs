mkuse!{use crate :: cell :: RefCell ;}
mkuse!{use crate :: sys :: thread_local :: guard ;}
mkitem!{# [thread_local] static DTORS : RefCell < Vec < (* mut u8 , unsafe extern "C" fn (* mut u8)) > > = RefCell :: new (Vec :: new ()) ;}

macro_rules! register_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function register in module {}", module_path!());
    };
}

mkfn!{
    register_introspect!();
    pub unsafe fn register (t : * mut u8 , dtor : unsafe extern "C" fn (* mut u8)) { let Ok (mut dtors) = DTORS . try_borrow_mut () else { rtabort ! ("the global allocator may not use TLS with destructors") ; } ; guard :: enable () ; dtors . push ((t , dtor)) ; }
}

macro_rules! run_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function run in module {}", module_path!());
    };
}

mkfn!{
    run_introspect!();
    # [doc = " The [`guard`] module contains platform-specific functions which will run this"] # [doc = " function on thread exit if [`guard::enable`] has been called."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " May only be run on thread exit to guarantee that there are no live references"] # [doc = " to TLS variables while they are destroyed."] pub unsafe fn run () { loop { let mut dtors = DTORS . borrow_mut () ; match dtors . pop () { Some ((t , dtor)) => { drop (dtors) ; unsafe { dtor (t) ; } } None => { * dtors = Vec :: new () ; break ; } } } }
}