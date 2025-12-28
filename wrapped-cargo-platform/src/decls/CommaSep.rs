macro_rules! CommaSep {
    () => {
        struct CommaSep < 'a , T > (& 'a [T]) ;
    };
}

CommaSep!();