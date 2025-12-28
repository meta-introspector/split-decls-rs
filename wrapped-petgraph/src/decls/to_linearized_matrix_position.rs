macro_rules! deps {
    () => {
        EdgeType!();
    };
}

macro_rules! to_linearized_matrix_position {
    () => {
        deps!();
        # [inline] fn to_linearized_matrix_position < Ty : EdgeType > (row : usize , column : usize , width : usize) -> usize { if Ty :: is_directed () { to_flat_square_matrix_position (row , column , width) } else { to_lower_triangular_matrix_position (row , column) } }
    };
}

to_linearized_matrix_position!()