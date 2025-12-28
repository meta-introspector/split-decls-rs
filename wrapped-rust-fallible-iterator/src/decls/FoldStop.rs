macro_rules! FoldStop {
    () => {
        enum FoldStop < T , E > { Break (T) , Err (E) , }
    };
}

FoldStop!();