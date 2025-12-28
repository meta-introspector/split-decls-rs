macro_rules! deps {
    () => {
        PeekingNext!();
    };
}

macro_rules! impl_404 {
    () => {
        deps!();
        # [cfg (feature = "use_alloc")] impl < T > PeekingNext for :: alloc :: vec :: IntoIter < T > { fn peeking_next < F > (& mut self , accept : F) -> Option < Self :: Item > where F : FnOnce (& Self :: Item) -> bool , { match accept (self . as_slice () . first () ?) { true => self . next () , false => None , } } }
    };
}

impl_404!();