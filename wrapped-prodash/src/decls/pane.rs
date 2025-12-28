macro_rules! deps {
    () => {
        Key!();
        Task!();
        State!();
    };
}

macro_rules! pane {
    () => {
        deps!();
        pub fn pane (entries : & [(Key , progress :: Task)] , mut bound : Rect , buf : & mut Buffer , state : & mut State) { state . task_offset = sanitize_offset (state . task_offset , entries . len () , bound . height) ; let needs_overflow_line = if entries . len () > bound . height as usize || (state . task_offset) . min (entries . len () as u16) > 0 { bound . height = bound . height . saturating_sub (1) ; true } else { false } ; state . task_offset = sanitize_offset (state . task_offset , entries . len () , bound . height) ; if entries . is_empty () { return ; } let initial_column_width = bound . width / 3 ; let desired_max_tree_draw_width = * state . next_tree_column_width . as_ref () . unwrap_or (& initial_column_width) ; { if initial_column_width >= MIN_TREE_WIDTH { let tree_bound = Rect { width : desired_max_tree_draw_width , .. bound } ; let computed = draw_tree (entries , buf , tree_bound , state . task_offset) ; state . last_tree_column_width = Some (computed) ; } else { state . last_tree_column_width = Some (0) ; } ; } { if let Some (tp) = state . throughput . as_mut () { tp . update_elapsed () ; } let progress_area = rect :: offset_x (bound , desired_max_tree_draw_width) ; draw_progress (entries , buf , progress_area , state . task_offset , state . throughput . as_mut () ,) ; if let Some (tp) = state . throughput . as_mut () { tp . reconcile (entries) ; } } if needs_overflow_line { let overflow_rect = Rect { y : bound . height + 1 , height : 1 , .. bound } ; draw_overflow (entries , buf , overflow_rect , desired_max_tree_draw_width , bound . height , state . task_offset ,) ; } }
    };
}

pane!();