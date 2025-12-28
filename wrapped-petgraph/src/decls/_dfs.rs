macro_rules! deps {
    () => {
        RecursionStep!();
        ArticulationPointTracker!();
    };
}

macro_rules! _dfs {
    () => {
        deps!();
        # [doc = " Helper that performs the required DFS in an iterative manner."] fn _dfs < G > (g : & G , target_node : usize , articulation_point_tracker : & mut ArticulationPointTracker) where G : IntoEdges + NodeIndexable , { let mut stack : Vec < RecursionStep > = vec ! [RecursionStep :: BaseStep (target_node)] ; let mut children_count : HashMap < usize , usize > = HashMap :: new () ; while let Some (recursion_step) = stack . pop () { match recursion_step { RecursionStep :: BaseStep (current_node) => { articulation_point_tracker . visited . insert (current_node) ; articulation_point_tracker . disc [current_node] = articulation_point_tracker . time ; articulation_point_tracker . low [current_node] = articulation_point_tracker . time ; articulation_point_tracker . time += 1 ; stack . push (RecursionStep :: RootMoreThanTwoChildrenCheck (current_node)) ; for edge in g . edges (g . from_index (current_node)) { let child_node = g . to_index (edge . target ()) ; stack . push (RecursionStep :: ProcessChildStep (current_node , child_node)) ; } } RecursionStep :: ProcessChildStep (current_node , child_node) => { if ! articulation_point_tracker . visited . contains (child_node) { articulation_point_tracker . parent [child_node] = current_node ; children_count . entry (current_node) . and_modify (| c | * c += 1) . or_insert (1) ; stack . push (RecursionStep :: NoBackEdgeConditionCheck (current_node , child_node ,)) ; stack . push (RecursionStep :: BaseStep (child_node)) ; } else if child_node != articulation_point_tracker . parent [current_node] { articulation_point_tracker . low [current_node] = min (articulation_point_tracker . low [current_node] , articulation_point_tracker . disc [child_node] ,) ; } } RecursionStep :: NoBackEdgeConditionCheck (current_node , child_node) => { articulation_point_tracker . low [current_node] = min (articulation_point_tracker . low [current_node] , articulation_point_tracker . low [child_node] ,) ; if articulation_point_tracker . parent [current_node] != usize :: MAX && articulation_point_tracker . low [child_node] >= articulation_point_tracker . disc [current_node] { articulation_point_tracker . articulation_points . insert (current_node) ; } } RecursionStep :: RootMoreThanTwoChildrenCheck (current_node) => { let child_count = children_count . get (& current_node) . cloned () . unwrap_or (0) ; if articulation_point_tracker . parent [current_node] == usize :: MAX && child_count > 1 { articulation_point_tracker . articulation_points . insert (current_node) ; } } } } }
    };
}

_dfs!();