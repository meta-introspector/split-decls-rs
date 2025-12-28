macro_rules! deps {
    () => {
        Sorting!();
        State!();
        Parents!();
    };
}

macro_rules! Simple {
    () => {
        deps!();
        # [doc = " A fast iterator over the ancestors of one or more starting commits."] pub struct Simple < Find , Predicate > { objects : Find , cache : Option < gix_commitgraph :: Graph > , predicate : Predicate , state : simple :: State , parents : Parents , sorting : simple :: Sorting , }
    };
}

Simple!()