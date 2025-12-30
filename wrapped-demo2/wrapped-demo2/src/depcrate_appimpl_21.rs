// Generated macro for impl_21 (impl)
macro_rules! Depcrate_appimpl_21 {
() => {
// Module: crate::app
// Provides: {"impl_21"}
// Dependencies: {}
impl App { fn render_title_bar (& self , area : Rect , buf : & mut Buffer) { let layout = Layout :: horizontal ([Constraint :: Min (0) , Constraint :: Length (43)]) ; let [title , tabs] = area . layout (& layout) ; Span :: styled ("Ratatui" , THEME . app_title) . render (title , buf) ; let titles = Tab :: iter () . map (Tab :: title) ; Tabs :: new (titles) . style (THEME . tabs) . highlight_style (THEME . tabs_selected) . select (self . tab as usize) . divider ("") . padding ("" , "") . render (tabs , buf) ; } fn render_selected_tab (& self , area : Rect , buf : & mut Buffer) { match self . tab { Tab :: About => self . about_tab . render (area , buf) , Tab :: Recipe => self . recipe_tab . render (area , buf) , Tab :: Email => self . email_tab . render (area , buf) , Tab :: Traceroute => self . traceroute_tab . render (area , buf) , Tab :: Weather => self . weather_tab . render (area , buf) , } ; } fn render_bottom_bar (area : Rect , buf : & mut Buffer) { let keys = [("H/←" , "Left") , ("L/→" , "Right") , ("K/↑" , "Up") , ("J/↓" , "Down") , ("D/Del" , "Destroy") , ("Q/Esc" , "Quit") ,] ; let spans = keys . iter () . flat_map (| (key , desc) | { let key = Span :: styled (format ! (" {key} ") , THEME . key_binding . key) ; let desc = Span :: styled (format ! (" {desc} ") , THEME . key_binding . description) ; [key , desc] }) . collect_vec () ; Line :: from (spans) . centered () . style ((Color :: Indexed (236) , Color :: Indexed (232))) . render (area , buf) ; } }
};
}
