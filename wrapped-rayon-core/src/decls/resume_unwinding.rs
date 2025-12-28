macro_rules! resume_unwinding {
    () => {
        pub (super) fn resume_unwinding (payload : Box < dyn Any + Send >) -> ! { panic :: resume_unwind (payload) }
    };
}

resume_unwinding!();