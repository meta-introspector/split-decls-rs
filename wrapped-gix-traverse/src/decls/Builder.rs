macro_rules! deps {
    () => {
        Sorting!();
        Topo!();
        Parents!();
    };
}

macro_rules! Builder {
    () => {
        deps!();
        # [doc = " Builder for [`Topo`]."] pub struct Builder < Find , Predicate > { commit_graph : Option < gix_commitgraph :: Graph > , find : Find , predicate : Predicate , sorting : Sorting , parents : Parents , tips : Vec < ObjectId > , ends : Vec < ObjectId > , }
    };
}

Builder!()