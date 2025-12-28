macro_rules! single_shrinker {
    () => {
        # [doc = " Creates a shrinker with a single element."] pub fn single_shrinker < A : 'static > (value : A) -> Box < dyn Iterator < Item = A > > { Box :: new (once (value)) }
    };
}

single_shrinker!()