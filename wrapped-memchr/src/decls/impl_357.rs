macro_rules! deps {
    () => {
        PrefilterState!();
    };
}

macro_rules! impl_357 {
    () => {
        deps!();
        impl PrefilterState { # [doc = " The minimum number of skip attempts to try before considering whether"] # [doc = " a prefilter is effective or not."] const MIN_SKIPS : u32 = 50 ; # [doc = " The minimum amount of bytes that skipping must average."] # [doc = ""] # [doc = " This value was chosen based on varying it and checking"] # [doc = " the microbenchmarks. In particular, this can impact the"] # [doc = " pathological/repeated-{huge,small} benchmarks quite a bit if it's set"] # [doc = " too low."] const MIN_SKIP_BYTES : u32 = 8 ; # [doc = " Create a fresh prefilter state."] # [inline] pub (crate) fn new () -> PrefilterState { PrefilterState { skips : 1 , skipped : 0 } } # [doc = " Update this state with the number of bytes skipped on the last"] # [doc = " invocation of the prefilter."] # [inline] fn update (& mut self , skipped : usize) { self . skips = self . skips . saturating_add (1) ; self . skipped = match u32 :: try_from (skipped) { Err (_) => core :: u32 :: MAX , Ok (skipped) => self . skipped . saturating_add (skipped) , } ; } # [doc = " Return true if and only if this state indicates that a prefilter is"] # [doc = " still effective."] # [inline] fn is_effective (& mut self) -> bool { if self . is_inert () { return false ; } if self . skips () < PrefilterState :: MIN_SKIPS { return true ; } if self . skipped >= PrefilterState :: MIN_SKIP_BYTES * self . skips () { return true ; } self . skips = 0 ; false } # [doc = " Returns true if the prefilter this state represents should no longer"] # [doc = " be used."] # [inline] fn is_inert (& self) -> bool { self . skips == 0 } # [doc = " Returns the total number of times the prefilter has been used."] # [inline] fn skips (& self) -> u32 { self . skips . saturating_sub (1) } }
    };
}

impl_357!()