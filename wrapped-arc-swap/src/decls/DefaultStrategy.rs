macro_rules! deps {
    () => {
        DefaultConfig!();
        HybridStrategy!();
        ArcSwap!();
        Guard!();
    };
}

macro_rules! DefaultStrategy {
    () => {
        deps!();
        # [doc = " The default strategy."] # [doc = ""] # [doc = " It is used by the type aliases [`ArcSwap`][crate::ArcSwap] and"] # [doc = " [`ArcSwapOption`][crate::ArcSwapOption]. Only the other strategies need to be used explicitly."] # [doc = ""] # [doc = " # Performance characteristics"] # [doc = ""] # [doc = " * It is optimized for read-heavy situations, with possibly many concurrent read accesses from"] # [doc = "   multiple threads. Readers don't contend each other at all."] # [doc = " * Readers are wait-free (with the exception of at most once in `usize::MAX / 4` accesses, which"] # [doc = "   is only lock-free)."] # [doc = " * Writers are lock-free."] # [doc = " * Reclamation is exact ‒ the resource is released as soon as possible (works like RAII, not"] # [doc = "   like a traditional garbage collector; can contain non-`'static` data)."] # [doc = ""] # [doc = " Each thread has a limited number of fast slots (currently 8, but the exact number is not"] # [doc = " guaranteed). If it holds at most that many [`Guard`]s at once, acquiring them is fast. Once"] # [doc = " these slots are used up (by holding to these many [`Guard`]s), acquiring more of them will be"] # [doc = " slightly slower, but still wait-free."] # [doc = ""] # [doc = " If you expect to hold a lot of \"handles\" to the data around, or hold onto it for a long time,"] # [doc = " you may want to prefer the [`load_full`][crate::ArcSwapAny::load_full] method."] # [doc = ""] # [doc = " The speed of the fast slots is in the ballpark of locking an *uncontented* mutex. The advantage"] # [doc = " over the mutex is the stability of speed in the face of contention from other threads ‒ while"] # [doc = " the performance of mutex goes rapidly down, the slowdown of running out of held slots or heavy"] # [doc = " concurrent writer thread in the area of single-digit multiples."] # [doc = ""] # [doc = " The ballpark benchmark figures (my older computer) are around these, but you're welcome to run"] # [doc = " the benchmarks in the git repository or write your own."] # [doc = ""] # [doc = " * Load (both uncontented and contented by other loads): ~30ns"] # [doc = " * `load_full`: ~50ns uncontented, goes up a bit with other `load_full` in other threads on the"] # [doc = "   same `Arc` value (~80-100ns)."] # [doc = " * Loads after running out of the slots ‒ about 10-20ns slower than `load_full`."] # [doc = " * Stores: Dependent on number of threads, but generally low microseconds."] # [doc = " * Loads with heavy concurrent writer (to the same `ArcSwap`): ~250ns."] # [doc = ""] # [doc = " [`load`]: crate::ArcSwapAny::load"] # [doc = " [`Guard`]: crate::Guard"] pub type DefaultStrategy = HybridStrategy < DefaultConfig > ;
    };
}

DefaultStrategy!()