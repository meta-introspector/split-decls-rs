macro_rules! extend_flat_square_matrix {
    () => {
        # [inline] fn extend_flat_square_matrix < T : Default > (node_adjacencies : & mut Vec < T > , old_node_capacity : usize , new_node_capacity : usize , exact : bool ,) -> usize { let new_node_capacity = if exact { new_node_capacity } else { const MIN_CAPACITY : usize = 4 ; cmp :: max (new_node_capacity . next_power_of_two () , MIN_CAPACITY) } ; ensure_len (node_adjacencies , new_node_capacity . pow (2)) ; for c in (1 .. old_node_capacity) . rev () { let pos = c * old_node_capacity ; let new_pos = c * new_node_capacity ; if pos + old_node_capacity <= new_pos { debug_assert ! (pos + old_node_capacity < node_adjacencies . len ()) ; debug_assert ! (new_pos + old_node_capacity < node_adjacencies . len ()) ; let ptr = node_adjacencies . as_mut_ptr () ; unsafe { let old = ptr . add (pos) ; let new = ptr . add (new_pos) ; core :: ptr :: swap_nonoverlapping (old , new , old_node_capacity) ; } } else { for i in (0 .. old_node_capacity) . rev () { node_adjacencies . as_mut_slice () . swap (pos + i , new_pos + i) ; } } } new_node_capacity }
    };
}

extend_flat_square_matrix!();