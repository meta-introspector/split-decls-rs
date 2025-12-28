macro_rules! deps {
    () => {
        Key!();
        Task!();
    };
}

macro_rules! draw_tree {
    () => {
        deps!();
        pub fn draw_tree (entries : & [(Key , Task)] , buf : & mut Buffer , bound : Rect , offset : u16) -> u16 { let mut max_prefix_len = 0 ; for (line , (entry_index , entry)) in entries . iter () . enumerate () . skip (offset as usize) . take (bound . height as usize) . enumerate () { let mut line_bound = rect :: line_bound (bound , line) ; line_bound . x = line_bound . x . saturating_sub (1) ; line_bound . width = line_bound . width . saturating_sub (1) ; let tree_prefix = format ! ("{} {} " , level_prefix (entries , entry_index) , entry . 1 . name) ; max_prefix_len = max_prefix_len . max (block_width (& tree_prefix)) ; let style = if entry . 1 . progress . is_none () { Style :: default () . add_modifier (Modifier :: BOLD) . into () } else { None } ; draw_text_with_ellipsis_nowrap (line_bound , buf , tree_prefix , style) ; } max_prefix_len }
    };
}

draw_tree!()