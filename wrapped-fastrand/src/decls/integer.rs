macro_rules! integer {
    () => {
        macro_rules ! integer { ($ t : tt , $ doc : tt) => { # [doc = $ doc] # [doc = ""] # [doc = " Panics if the range is empty."] # [inline] pub fn $ t (range : impl RangeBounds <$ t >) -> $ t { with_rng (| r | r .$ t (range)) } } ; }
    };
}

integer!()