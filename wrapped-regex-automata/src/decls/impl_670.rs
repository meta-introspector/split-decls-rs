macro_rules! deps {
    () => {
        TryCapturesIter!();
        CapturesIter!();
    };
}

macro_rules! impl_670 {
    () => {
        deps!();
        # [cfg (feature = "alloc")] impl < 'h , F > TryCapturesIter < 'h , F > { # [doc = " Return an infallible version of this iterator."] # [doc = ""] # [doc = " Any item yielded that corresponds to an error results in a panic. This"] # [doc = " is useful if your underlying regex engine is configured in a way that"] # [doc = " it is guaranteed to never return an error."] pub fn infallible (self) -> CapturesIter < 'h , F > { CapturesIter (self) } }
    };
}

impl_670!()