macro_rules! deps {
    () => {
        EdgeType!();
    };
}

macro_rules! extend_linearized_matrix {
    () => {
        deps!();
        # [inline] fn extend_linearized_matrix < Ty : EdgeType , T : Default > (node_adjacencies : & mut Vec < T > , old_node_capacity : usize , new_capacity : usize , exact : bool ,) -> usize { if old_node_capacity >= new_capacity { return old_node_capacity ; } if Ty :: is_directed () { extend_flat_square_matrix (node_adjacencies , old_node_capacity , new_capacity , exact) } else { extend_lower_triangular_matrix (node_adjacencies , new_capacity) } }
    };
}

extend_linearized_matrix!()