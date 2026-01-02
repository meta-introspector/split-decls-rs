mkuse!{use crate :: cell :: Cell ;}
mkuse!{use crate :: sys :: pal :: abi ;}
mkuse!{use crate :: sys :: pal :: itron :: task ;}
mkuse!{use crate :: sys :: thread_local :: destructors ;}

macro_rules! enable_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function enable in module {}", module_path!());
    };
}

mkfn!{
    enable_introspect!();
    pub fn enable () { # [thread_local] static REGISTERED : Cell < bool > = Cell :: new (false) ; if ! REGISTERED . replace (true) { let tid = task :: current_task_id_aborting () ; unsafe { abi :: SOLID_TLS_AddDestructor (tid as i32 , tls_dtor) } ; } unsafe extern "C" fn tls_dtor (_unused : * mut u8) { unsafe { destructors :: run () ; crate :: rt :: thread_cleanup () ; } } }
}