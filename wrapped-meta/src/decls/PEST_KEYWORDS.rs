macro_rules! PEST_KEYWORDS {
    () => {
        static PEST_KEYWORDS : LazyLock < HashSet < & 'static str > > = LazyLock :: new (| | { ["_" , "ANY" , "DROP" , "EOI" , "PEEK" , "PEEK_ALL" , "POP" , "POP_ALL" , "PUSH" , "SOI" ,] . iter () . cloned () . collect () }) ;
    };
}

PEST_KEYWORDS!()