macro_rules! deps {
    () => {
        LocalPool!();
    };
}

macro_rules! block_on {
    () => {
        deps!();
        # [doc = " Run a future to completion on the current thread."] # [doc = ""] # [doc = " This function will block the caller until the given future has completed."] # [doc = ""] # [doc = " Use a [`LocalPool`] if you need finer-grained control over spawned tasks."] pub fn block_on < F : Future > (f : F) -> F :: Output { let mut f = pin ! (f) ; run_executor (| cx | f . as_mut () . poll (cx)) }
    };
}

block_on!();