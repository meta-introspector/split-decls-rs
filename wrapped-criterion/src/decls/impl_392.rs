macro_rules! deps {
    () => {
        BatchSize!();
    };
}

macro_rules! impl_392 {
    () => {
        deps!();
        impl BatchSize { # [doc = " Convert to a number of iterations per batch."] # [doc = ""] # [doc = " We try to do a constant number of batches regardless of the number of iterations in this"] # [doc = " sample. If the measurement overhead is roughly constant regardless of the number of"] # [doc = " iterations the analysis of the results later will have an easier time separating the"] # [doc = " measurement overhead from the benchmark time."] fn iters_per_batch (self , iters : u64) -> u64 { match self { BatchSize :: SmallInput => (iters + 10 - 1) / 10 , BatchSize :: LargeInput => (iters + 1000 - 1) / 1000 , BatchSize :: PerIteration => 1 , BatchSize :: NumBatches (batches) => (iters + batches - 1) / batches , BatchSize :: NumIterations (size) => size , BatchSize :: __NonExhaustive => panic ! ("__NonExhaustive is not a valid BatchSize.") , } } }
    };
}

impl_392!()