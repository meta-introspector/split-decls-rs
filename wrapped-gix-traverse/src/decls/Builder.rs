macro_rules! deps {
    () => {
        Topo!();
        Parents!();
        Sorting!();
    };
}

macro_rules! Builder {
    () => {
        deps!();
        # [doc = " Builder for [`Topo`]."] pub struct Builder < Find , Predicate > { commit_graph : Option < gix_commitgraph :: Graph > , find : Find , predicate : Predicate , sorting : Sorting , parents : Parents , tips : Vec < ObjectId > , ends : Vec < ObjectId > , }
    };
}

Builder!();