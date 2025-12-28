macro_rules! deps {
    () => {
        Teddy!();
        Searcher!();
        RabinKarp!();
    };
}

macro_rules! SearchKind {
    () => {
        deps!();
        # [derive (Clone , Debug)] enum SearchKind { Teddy (teddy :: Searcher) , RabinKarp , }
    };
}

SearchKind!();