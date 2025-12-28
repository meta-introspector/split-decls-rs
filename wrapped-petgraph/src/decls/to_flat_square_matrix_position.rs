macro_rules! to_flat_square_matrix_position {
    () => {
        # [inline] fn to_flat_square_matrix_position (row : usize , column : usize , width : usize) -> usize { row * width + column }
    };
}

to_flat_square_matrix_position!()