macro_rules! deps {
    () => {
        ParallelBridge!();
        Iter!();
    };
}

macro_rules! IterBridge {
    () => {
        deps!();
        # [doc = " `IterBridge` is a parallel iterator that wraps a sequential iterator."] # [doc = ""] # [doc = " This type is created when using the `par_bridge` method on `ParallelBridge`. See the"] # [doc = " [`ParallelBridge`] documentation for details."] # [derive (Debug , Clone)] pub struct IterBridge < Iter > { iter : Iter , }
    };
}

IterBridge!()