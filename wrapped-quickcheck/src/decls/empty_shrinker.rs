macro_rules! empty_shrinker {
    () => {
        # [doc = " Creates a shrinker with zero elements."] pub fn empty_shrinker < A : 'static > () -> Box < dyn Iterator < Item = A > > { Box :: new (empty ()) }
    };
}

empty_shrinker!()