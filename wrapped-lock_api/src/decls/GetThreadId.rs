macro_rules! GetThreadId {
    () => {
        # [doc = " Helper trait which returns a non-zero thread ID."] # [doc = ""] # [doc = " The simplest way to implement this trait is to return the address of a"] # [doc = " thread-local variable."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Implementations of this trait must ensure that no two active threads share"] # [doc = " the same thread ID. However the ID of a thread that has exited can be"] # [doc = " re-used since that thread is no longer active."] pub unsafe trait GetThreadId { # [doc = " Initial value."] # [allow (clippy :: declare_interior_mutable_const)] const INIT : Self ; # [doc = " Returns a non-zero thread ID which identifies the current thread of"] # [doc = " execution."] fn nonzero_thread_id (& self) -> NonZeroUsize ; }
    };
}

GetThreadId!()