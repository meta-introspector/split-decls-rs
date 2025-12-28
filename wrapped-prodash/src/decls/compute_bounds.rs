macro_rules! compute_bounds {
    () => {
        fn compute_bounds (line : Rect , max_origin_width : u16) -> (Option < Rect > , Option < Rect > , Option < Rect > , Rect) { let vertical_line_width = VERTICAL_LINE . width () as u16 ; let mythical_offset_we_should_not_need = 1 ; let time_bound = Rect { width : DATE_TIME_HMS as u16 + vertical_line_width , .. line } ; let mut cursor = time_bound . width + mythical_offset_we_should_not_need ; let level_bound = Rect { x : cursor , width : LEVEL_TEXT_WIDTH + vertical_line_width , .. line } ; cursor += level_bound . width ; let origin_bound = Rect { x : cursor , width : max_origin_width + vertical_line_width , .. line } ; cursor += origin_bound . width ; let message_bound = rect :: intersect (rect :: offset_x (line , cursor) , line) ; if message_bound . width < 30 { return (None , None , None , line) ; } (Some (time_bound) , Some (level_bound) , Some (origin_bound) , message_bound) }
    };
}

compute_bounds!()