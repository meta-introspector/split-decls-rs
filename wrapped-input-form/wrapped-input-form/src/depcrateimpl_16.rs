// Generated macro for impl_16 (impl)
macro_rules! Depcrateimpl_16 {
() => {
// Module: crate
// Provides: {"impl_16"}
// Dependencies: {}
impl InputForm { fn on_key_press (& mut self , event : KeyEvent) { match event . code { KeyCode :: Tab => self . focus = self . focus . next () , _ => match self . focus { Focus :: FirstName => self . first_name . on_key_press (event) , Focus :: LastName => self . last_name . on_key_press (event) , Focus :: Age => self . age . on_key_press (event) , } , } } # [doc = " Render the form with the current focus."] # [doc = ""] # [doc = " The cursor is placed at the end of the focused field."] fn render (& self , frame : & mut Frame) { let layout = Layout :: vertical (Constraint :: from_lengths ([1 , 1 , 1])) ; let [first_name_area , last_name_area , age_area] = frame . area () . layout (& layout) ; frame . render_widget (& self . first_name , first_name_area) ; frame . render_widget (& self . last_name , last_name_area) ; frame . render_widget (& self . age , age_area) ; let cursor_position = match self . focus { Focus :: FirstName => first_name_area . offset (self . first_name . cursor_offset ()) , Focus :: LastName => last_name_area . offset (self . last_name . cursor_offset ()) , Focus :: Age => age_area . offset (self . age . cursor_offset ()) , } ; frame . set_cursor_position (cursor_position) ; } }
};
}
