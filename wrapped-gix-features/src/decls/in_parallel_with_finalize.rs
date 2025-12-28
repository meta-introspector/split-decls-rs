macro_rules! deps {
    () => {
        Reduce!();
        Read!();
    };
}

macro_rules! in_parallel_with_finalize {
    () => {
        deps!();
        # [doc = " Read items from `input` and `consume` them in multiple threads,"] # [doc = " whose output is collected by a `reducer`. Its task is to"] # [doc = " aggregate these outputs into the final result returned by this function with the benefit of not having to be thread-safe."] # [doc = " Call `finalize` to finish the computation, once per thread, if there was no error sending results earlier."] # [doc = ""] # [doc = " * if `thread_limit` is `Some`, the given number of threads will be used. If `None`, all logical cores will be used."] # [doc = " * `new_thread_state(thread_number) -> State` produces thread-local state once per thread to be passed to `consume`"] # [doc = " * `consume(Item, &mut State) -> Output` produces an output given an input obtained by `input` along with mutable state initially"] # [doc = "   created by `new_thread_state(…)`."] # [doc = " * `finalize(State) -> Output` is called to potentially process remaining work that was placed in `State`."] # [doc = " * For `reducer`, see the [`Reduce`] trait"] # [cfg (not (feature = "parallel"))] pub fn in_parallel_with_finalize < I , S , O , R > (input : impl Iterator < Item = I > , _thread_limit : Option < usize > , new_thread_state : impl FnOnce (usize) -> S , mut consume : impl FnMut (I , & mut S) -> O , finalize : impl FnOnce (S) -> O + Send + Clone , mut reducer : R ,) -> Result < < R as Reduce > :: Output , < R as Reduce > :: Error > where R : Reduce < Input = O > , { let mut state = new_thread_state (0) ; for item in input { drop (reducer . feed (consume (item , & mut state)) ?) ; } reducer . feed (finalize (state)) ? ; reducer . finalize () }
    };
}

in_parallel_with_finalize!()