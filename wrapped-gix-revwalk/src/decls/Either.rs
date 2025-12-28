macro_rules! Either {
    () => {
        enum Either < T , U > { Left (T) , Right (U) , }
    };
}

Either!();