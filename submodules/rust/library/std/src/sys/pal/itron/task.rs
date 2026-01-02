mkuse!{use super :: abi ;}
mkuse!{use super :: error :: { ItronError , fail , fail_aborting } ;}
mkuse!{use crate :: mem :: MaybeUninit ;}

macro_rules! current_task_id_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function current_task_id in module {}", module_path!());
    };
}

mkfn!{
    current_task_id_introspect!();
    # [doc = " Gets the ID of the task in Running state. Panics on failure."] # [inline] pub fn current_task_id () -> abi :: ID { try_current_task_id () . unwrap_or_else (| e | fail (e , & "get_tid")) }
}

macro_rules! current_task_id_aborting_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function current_task_id_aborting in module {}", module_path!());
    };
}

mkfn!{
    current_task_id_aborting_introspect!();
    # [doc = " Gets the ID of the task in Running state. Aborts on failure."] # [inline] pub fn current_task_id_aborting () -> abi :: ID { try_current_task_id () . unwrap_or_else (| e | fail_aborting (e , & "get_tid")) }
}

macro_rules! try_current_task_id_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function try_current_task_id in module {}", module_path!());
    };
}

mkfn!{
    try_current_task_id_introspect!();
    # [doc = " Gets the ID of the task in Running state."] # [inline] pub fn try_current_task_id () -> Result < abi :: ID , ItronError > { unsafe { let mut out = MaybeUninit :: uninit () ; ItronError :: err_if_negative (abi :: get_tid (out . as_mut_ptr ())) ? ; Ok (out . assume_init ()) } }
}

macro_rules! task_priority_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function task_priority in module {}", module_path!());
    };
}

mkfn!{
    task_priority_introspect!();
    # [doc = " Gets the specified task's priority. Panics on failure."] # [inline] pub fn task_priority (task : abi :: ID) -> abi :: PRI { try_task_priority (task) . unwrap_or_else (| e | fail (e , & "get_pri")) }
}

macro_rules! try_task_priority_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function try_task_priority in module {}", module_path!());
    };
}

mkfn!{
    try_task_priority_introspect!();
    # [doc = " Gets the specified task's priority."] # [inline] pub fn try_task_priority (task : abi :: ID) -> Result < abi :: PRI , ItronError > { unsafe { let mut out = MaybeUninit :: uninit () ; ItronError :: err_if_negative (abi :: get_pri (task , out . as_mut_ptr ())) ? ; Ok (out . assume_init ()) } }
}