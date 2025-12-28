macro_rules! deps {
    () => {
        Scope!();
        SharedVec!();
        WaitGroup!();
    };
}

macro_rules! scope {
    () => {
        deps!();
        # [doc = " Creates a new scope for spawning threads."] # [doc = ""] # [doc = " All child threads that haven't been manually joined will be automatically joined just before"] # [doc = " this function invocation ends. If all joined threads have successfully completed, `Ok` is"] # [doc = " returned with the return value of `f`. If any of the joined threads has panicked, an `Err` is"] # [doc = " returned containing errors from panicked threads. Note that if panics are implemented by"] # [doc = " aborting the process, no error is returned; see the notes of [std::panic::catch_unwind]."] # [doc = ""] # [doc = " **Note:** Since Rust 1.63, this function is soft-deprecated in favor of the more efficient [`std::thread::scope`]."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use crossbeam_utils::thread;"] # [doc = ""] # [doc = " let var = vec![1, 2, 3];"] # [doc = ""] # [doc = " thread::scope(|s| {"] # [doc = "     s.spawn(|_| {"] # [doc = "         println!(\"A child thread borrowing `var`: {:?}\", var);"] # [doc = "     });"] # [doc = " }).unwrap();"] # [doc = " ```"] pub fn scope < 'env , F , R > (f : F) -> thread :: Result < R > where F : FnOnce (& Scope < 'env >) -> R , { struct AbortOnPanic ; impl Drop for AbortOnPanic { fn drop (& mut self) { if thread :: panicking () { std :: process :: abort () ; } } } let wg = WaitGroup :: new () ; let scope = Scope :: < 'env > { handles : SharedVec :: default () , wait_group : wg . clone () , _marker : PhantomData , } ; let result = panic :: catch_unwind (panic :: AssertUnwindSafe (| | f (& scope))) ; let guard = AbortOnPanic ; drop (scope . wait_group) ; wg . wait () ; let panics : Vec < _ > = scope . handles . lock () . unwrap () . drain (..) . filter_map (| handle | handle . lock () . unwrap () . take ()) . filter_map (| handle | handle . join () . err ()) . collect () ; mem :: forget (guard) ; match result { Err (err) => panic :: resume_unwind (err) , Ok (res) => { if panics . is_empty () { Ok (res) } else { Err (Box :: new (panics)) } } } }
    };
}

scope!()