macro_rules! IteratorType {
    () => {
        type IteratorType < I > = std :: iter :: Peekable < I > ;
    };
}

IteratorType!();