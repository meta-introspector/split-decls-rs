macro_rules! to_lower_triangular_matrix_position {
    () => {
        # [inline] fn to_lower_triangular_matrix_position (row : usize , column : usize) -> usize { let (row , column) = if row > column { (row , column) } else { (column , row) } ; (row * (row + 1)) / 2 + column }
    };
}

to_lower_triangular_matrix_position!();