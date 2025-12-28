macro_rules! extend_lower_triangular_matrix {
    () => {
        # [inline] fn extend_lower_triangular_matrix < T : Default > (node_adjacencies : & mut Vec < T > , new_capacity : usize ,) -> usize { let max_node = new_capacity - 1 ; let max_pos = to_lower_triangular_matrix_position (max_node , max_node) ; ensure_len (node_adjacencies , max_pos + 1) ; new_capacity }
    };
}

extend_lower_triangular_matrix!()