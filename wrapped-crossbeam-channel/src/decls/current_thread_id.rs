macro_rules! current_thread_id {
    () => {
        # [doc = " Returns the id of the current thread."] # [inline] fn current_thread_id () -> ThreadId { std :: thread_local ! { # [doc = " Cached thread-local id."] static THREAD_ID : ThreadId = thread :: current () . id () ; } THREAD_ID . try_with (| id | * id) . unwrap_or_else (| _ | thread :: current () . id ()) }
    };
}

current_thread_id!()