mkuse!{use super :: abi ;}
mkuse!{use super :: error :: expect_success_aborting ;}
mkuse!{use super :: time :: with_tmos ;}
mkuse!{use crate :: time :: Duration ;}
mkitem!{pub type ThreadId = abi :: ID ;}
mkuse!{pub use super :: task :: current_task_id_aborting as current ;}

macro_rules! park_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function park in module {}", module_path!());
    };
}

mkfn!{
    park_introspect!();
    pub fn park (_hint : usize) { match unsafe { abi :: slp_tsk () } { abi :: E_OK | abi :: E_RLWAI => { } err => { expect_success_aborting (err , & "slp_tsk") ; } } }
}

macro_rules! park_timeout_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function park_timeout in module {}", module_path!());
    };
}

mkfn!{
    park_timeout_introspect!();
    pub fn park_timeout (dur : Duration , _hint : usize) { match with_tmos (dur , | tmo | unsafe { abi :: tslp_tsk (tmo) }) { abi :: E_OK | abi :: E_RLWAI | abi :: E_TMOUT => { } err => { expect_success_aborting (err , & "tslp_tsk") ; } } }
}

macro_rules! unpark_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function unpark in module {}", module_path!());
    };
}

mkfn!{
    unpark_introspect!();
    pub fn unpark (id : ThreadId , _hint : usize) { match unsafe { abi :: wup_tsk (id) } { abi :: E_OK | abi :: E_NOEXS | abi :: E_OBJ | abi :: E_QOVR => { } err => { expect_success_aborting (err , & "wup_tsk") ; } } }
}