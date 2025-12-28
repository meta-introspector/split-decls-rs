macro_rules! deps {
    () => {
        Yield!();
        WorkerThread!();
    };
}

macro_rules! yield_now {
    () => {
        deps!();
        # [doc = " Cooperatively yields execution to Rayon."] # [doc = ""] # [doc = " If the current thread is part of a rayon thread pool, this looks for a"] # [doc = " single unit of pending work in the pool, then executes it. Completion of"] # [doc = " that work might include nested work or further work stealing."] # [doc = ""] # [doc = " This is similar to [`std::thread::yield_now()`], but does not literally make"] # [doc = " that call. If you are implementing a polling loop, you may want to also"] # [doc = " yield to the OS scheduler yourself if no Rayon work was found."] # [doc = ""] # [doc = " Returns `Some(Yield::Executed)` if anything was executed, `Some(Yield::Idle)` if"] # [doc = " nothing was available, or `None` if this thread is not part of any pool at all."] pub fn yield_now () -> Option < Yield > { unsafe { let thread = WorkerThread :: current () . as_ref () ? ; Some (thread . yield_now ()) } }
    };
}

yield_now!();