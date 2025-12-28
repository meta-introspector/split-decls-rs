macro_rules! deps {
    () => {
        PeekingNext!();
    };
}

macro_rules! impl_406 {
    () => {
        deps!();
        # [cfg (feature = "use_alloc")] impl < 'a > PeekingNext for :: alloc :: string :: Drain < 'a > { fn peeking_next < F > (& mut self , accept : F) -> Option < Self :: Item > where F : FnOnce (& Self :: Item) -> bool , { match accept (& self . as_str () . chars () . next () ?) { true => self . next () , false => None , } } }
    };
}

impl_406!()