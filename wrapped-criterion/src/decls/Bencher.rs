macro_rules! deps {
    () => {
        Measurement!();
        WallTime!();
        Duration!();
        BatchSize!();
    };
}

macro_rules! Bencher {
    () => {
        deps!();
        # [doc = " Timer struct used to iterate a benchmarked function and measure the runtime."] # [doc = ""] # [doc = " This struct provides different timing loops as methods. Each timing loop provides a different"] # [doc = " way to time a routine and each has advantages and disadvantages."] # [doc = ""] # [doc = " * If you want to do the iteration and measurement yourself (eg. passing the iteration count"] # [doc = "   to a separate process), use [`iter_custom`]."] # [doc = " * If your routine requires no per-iteration setup and returns a value with an expensive `drop`"] # [doc = "   method, use [`iter_with_large_drop`]."] # [doc = " * If your routine requires some per-iteration setup that shouldn't be timed, use [`iter_batched`]"] # [doc = "   or [`iter_batched_ref`]. See [`BatchSize`] for a discussion of batch sizes."] # [doc = "   If the setup value implements `Drop` and you don't want to include the `drop` time in the"] # [doc = "   measurement, use [`iter_batched_ref`], otherwise use [`iter_batched`]. These methods are also"] # [doc = "   suitable for benchmarking routines which return a value with an expensive `drop` method,"] # [doc = "   but are more complex than [`iter_with_large_drop`]."] # [doc = " * Otherwise, use [`iter`]."] # [doc = ""] # [doc = " [`iter`]: Bencher::iter"] # [doc = " [`iter_custom`]: Bencher::iter_custom"] # [doc = " [`iter_with_large_drop`]: Bencher::iter_with_large_drop"] # [doc = " [`iter_batched`]: Bencher::iter_batched"] # [doc = " [`iter_batched_ref`]: Bencher::iter_batched_ref"] pub struct Bencher < 'a , M : Measurement = WallTime > { pub (crate) iterated : bool , pub (crate) iters : u64 , pub (crate) value : M :: Value , pub (crate) measurement : & 'a M , pub (crate) elapsed_time : Duration , }
    };
}

Bencher!()