macro_rules! deps {
    () => {
        IndexedParallelIterator!();
    };
}

macro_rules! StepBy {
    () => {
        deps!();
        # [doc = " `StepBy` is an iterator that skips `n` elements between each yield, where `n` is the given step."] # [doc = " This struct is created by the [`step_by()`] method on [`IndexedParallelIterator`]"] # [doc = ""] # [doc = " [`step_by()`]: IndexedParallelIterator::step_by()"] # [must_use = "iterator adaptors are lazy and do nothing unless consumed"] # [derive (Debug , Clone)] pub struct StepBy < I > { base : I , step : usize , }
    };
}

StepBy!()