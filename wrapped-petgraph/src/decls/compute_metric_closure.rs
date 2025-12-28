macro_rules! deps {
    () => {
        Measure!();
    };
}

macro_rules! compute_metric_closure {
    () => {
        deps!();
        fn compute_metric_closure < G > (graph : G , terminals : & [G :: NodeId] ,) -> HashMap < (usize , usize) , G :: EdgeWeight > where G : Data + IntoNodeReferences + NodeIndexable + Visitable + IntoEdges , G :: EdgeWeight : Copy + Measure , G :: NodeId : PartialOrd + Eq + Hash , { let mut closure = HashMap :: new () ; for (i , node_id_1) in terminals . iter () . enumerate () { for node_id_2 in terminals . iter () . skip (i + 1) { closure . insert ((graph . to_index (* node_id_1) , graph . to_index (* node_id_2)) , compute_shortest_path_length (graph , * node_id_1 , * node_id_2) ,) ; } } closure }
    };
}

compute_metric_closure!();