macro_rules! deps {
    () => {
        Scope!();
    };
}

macro_rules! in_place_scope {
    () => {
        deps!();
        # [doc = " Creates a \"fork-join\" scope `s` and invokes the closure with a"] # [doc = " reference to `s`. This closure can then spawn asynchronous tasks"] # [doc = " into `s`. Those tasks may run asynchronously with respect to the"] # [doc = " closure; they may themselves spawn additional tasks into `s`. When"] # [doc = " the closure returns, it will block until all tasks that have been"] # [doc = " spawned into `s` complete."] # [doc = ""] # [doc = " This is just like `scope()` except the closure runs on the same thread"] # [doc = " that calls `in_place_scope()`. Only work that it spawns runs in the"] # [doc = " thread pool."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " If a panic occurs, either in the closure given to `in_place_scope()` or in"] # [doc = " any of the spawned jobs, that panic will be propagated and the"] # [doc = " call to `in_place_scope()` will panic. If multiple panics occurs, it is"] # [doc = " non-deterministic which of their panic values will propagate."] # [doc = " Regardless, once a task is spawned using `scope.spawn()`, it will"] # [doc = " execute, even if the spawning task should later panic. `in_place_scope()`"] # [doc = " returns once all spawned jobs have completed, and any panics are"] # [doc = " propagated at that point."] pub fn in_place_scope < 'scope , OP , R > (op : OP) -> R where OP : FnOnce (& Scope < 'scope >) -> R , { do_in_place_scope (None , op) }
    };
}

in_place_scope!();