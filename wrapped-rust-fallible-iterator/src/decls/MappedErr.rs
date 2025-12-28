macro_rules! MappedErr {
    () => {
        enum MappedErr < T , U > { It (T) , Fold (U) , }
    };
}

MappedErr!()