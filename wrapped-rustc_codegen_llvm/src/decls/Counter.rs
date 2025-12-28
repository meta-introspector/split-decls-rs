macro_rules! deps {
    () => {
        CounterKind!();
    };
}

macro_rules! Counter {
    () => {
        deps!();
        # [doc = " A reference to an instance of an abstract \"counter\" that will yield a value in a coverage"] # [doc = " report. Note that `id` has different interpretations, depending on the `kind`:"] # [doc = "   * For `CounterKind::Zero`, `id` is assumed to be `0`"] # [doc = "   * For `CounterKind::CounterValueReference`,  `id` matches the `counter_id` of the injected"] # [doc = "     instrumentation counter (the `index` argument to the LLVM intrinsic"] # [doc = "     `instrprof.increment()`)"] # [doc = "   * For `CounterKind::Expression`, `id` is the index into the coverage map's array of"] # [doc = "     counter expressions."] # [doc = ""] # [doc = " Corresponds to struct `llvm::coverage::Counter`."] # [doc = ""] # [doc = " Must match the layout of `LLVMRustCounter`."] # [derive (Copy , Clone , Debug)] # [repr (C)] pub (crate) struct Counter { pub (crate) kind : CounterKind , id : u32 , }
    };
}

Counter!();