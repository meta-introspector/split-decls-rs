macro_rules! deps {
    () => {
        FinderRev!();
        SearcherRevKind!();
    };
}

macro_rules! SearcherRev {
    () => {
        deps!();
        # [doc = " A reverse substring searcher."] # [derive (Clone , Debug)] pub (crate) struct SearcherRev { kind : SearcherRevKind , rabinkarp : rabinkarp :: FinderRev , }
    };
}

SearcherRev!()