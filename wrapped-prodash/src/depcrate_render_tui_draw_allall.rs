// Generated macro for all (function)
macro_rules! Depcrate_render_tui_draw_allall {
() => {
// Module: crate::render::tui::draw::all
// Provides: {"all"}
// Dependencies: {}
pub (crate) fn all (state : & mut State , interrupt_mode : InterruptDrawInfo , entries : & [(Key , Task)] , messages : & [Message] , bound : Rect , buf : & mut Buffer ,) { let (bound , info_pane) = compute_info_bound (bound , if state . hide_info { & [] } else { & state . information } , state . maximize_info ,) ; let bold = Style :: default () . add_modifier (Modifier :: BOLD) ; let window = Block :: default () . title (Span :: styled (state . title . as_str () , bold)) . borders (Borders :: ALL) ; let inner_area = window . inner (bound) ; window . render (bound , buf) ; if bound . width < 4 || bound . height < 4 { return ; } let border_width = 1 ; draw :: progress :: headline (entries , interrupt_mode , state . duration_per_frame , buf , rect :: offset_x (Rect { height : 1 , width : bound . width . saturating_sub (border_width) , .. bound } , block_width (& state . title) + (border_width * 2) ,) ,) ; let (progress_pane , messages_pane) = compute_pane_bounds (if state . hide_messages { & [] } else { messages } , inner_area , state . messages_fullscreen ,) ; draw :: progress :: pane (entries , progress_pane , buf , state) ; if let Some (messages_pane) = messages_pane { draw :: messages :: pane (messages , messages_pane , Rect { width : messages_pane . width + 2 , .. rect :: line_bound (bound , bound . height . saturating_sub (1) as usize) } , & mut state . message_offset , buf ,) ; } if let Some (info_pane) = info_pane { draw :: information :: pane (& state . information , info_pane , buf) ; } }
};
}
