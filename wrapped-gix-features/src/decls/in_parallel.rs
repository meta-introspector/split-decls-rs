macro_rules! deps {
    () => {
        Read!();
        Reduce!();
    };
}

macro_rules! in_parallel {
    () => {
        deps!();
        # [doc = " Read items from `input` and `consume` them in a single thread, producing an output to be collected by a `reducer`,"] # [doc = " whose task it is to aggregate these outputs into the final result returned by this function."] # [doc = ""] # [doc = " * `new_thread_state(thread_number) -> State` produces thread-local state once per thread to be passed to `consume`"] # [doc = " * `consume(Item, &mut State) -> Output` produces an output given an input along with mutable state."] # [doc = " * For `reducer`, see the [`Reduce`] trait"] # [doc = " * `thread_limit` has no effect as everything is run on the main thread, but is present to keep the signature"] # [doc = "   similar to the parallel version."] # [doc = ""] # [doc = " **This serial version performing all calculations on the current thread.**"] pub fn in_parallel < I , S , O , R > (input : impl Iterator < Item = I > , _thread_limit : Option < usize > , new_thread_state : impl FnOnce (usize) -> S , mut consume : impl FnMut (I , & mut S) -> O , mut reducer : R ,) -> Result < < R as Reduce > :: Output , < R as Reduce > :: Error > where R : Reduce < Input = O > , { let mut state = new_thread_state (0) ; for item in input { drop (reducer . feed (consume (item , & mut state)) ?) ; } reducer . finalize () }
    };
}

in_parallel!();