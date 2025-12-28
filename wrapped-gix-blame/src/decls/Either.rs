macro_rules! Either {
    () => {
        # [derive (Debug)] pub (crate) enum Either < T , U > { Left (T) , Right (U) , }
    };
}

Either!();