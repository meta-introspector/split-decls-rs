macro_rules! deps {
    () => {
        Step!();
    };
}

macro_rules! draw_spinner {
    () => {
        deps!();
        fn draw_spinner (buf : & mut Buffer , bound : Rect , step : Step , seed : usize , color : Color) { if bound . width == 0 { return ; } let x = bound . x + ((step + seed) % bound . width as usize) as u16 ; let width = 5 ; let bound = rect :: intersect (Rect { x , width , .. bound } , bound) ; tui_react :: fill_background (bound , buf , color) ; }
    };
}

draw_spinner!()