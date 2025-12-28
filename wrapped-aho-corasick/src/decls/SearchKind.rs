macro_rules! deps {
    () => {
        RabinKarp!();
        Searcher!();
        Teddy!();
    };
}

macro_rules! SearchKind {
    () => {
        deps!();
        # [derive (Clone , Debug)] enum SearchKind { Teddy (teddy :: Searcher) , RabinKarp , }
    };
}

SearchKind!()