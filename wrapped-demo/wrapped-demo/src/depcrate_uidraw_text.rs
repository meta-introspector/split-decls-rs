// Generated macro for draw_text (function)
macro_rules! Depcrate_uidraw_text {
() => {
// Module: crate::ui
// Provides: {"draw_text"}
// Dependencies: {}
fn draw_text (frame : & mut Frame , area : Rect) { let text = vec ! [text :: Line :: from ("This is a paragraph with several lines. You can change style your text the way you want" ,) , text :: Line :: from ("") , text :: Line :: from (vec ! [Span :: from ("For example: ") , Span :: styled ("under" , Style :: default () . fg (Color :: Red)) , Span :: raw (" ") , Span :: styled ("the" , Style :: default () . fg (Color :: Green)) , Span :: raw (" ") , Span :: styled ("rainbow" , Style :: default () . fg (Color :: Blue)) , Span :: raw (".") ,]) , text :: Line :: from (vec ! [Span :: raw ("Oh and if you didn't ") , Span :: styled ("notice" , Style :: default () . add_modifier (Modifier :: ITALIC)) , Span :: raw (" you can ") , Span :: styled ("automatically" , Style :: default () . add_modifier (Modifier :: BOLD) ,) , Span :: raw (" ") , Span :: styled ("wrap" , Style :: default () . add_modifier (Modifier :: REVERSED)) , Span :: raw (" your ") , Span :: styled ("text" , Style :: default () . add_modifier (Modifier :: UNDERLINED)) , Span :: raw (".") ,]) , text :: Line :: from ("One more thing is that it should display unicode characters: 10€") ,] ; let block = Block :: bordered () . title (Span :: styled ("Footer" , Style :: default () . fg (Color :: Magenta) . add_modifier (Modifier :: BOLD) ,)) ; let paragraph = Paragraph :: new (text) . block (block) . wrap (Wrap { trim : true }) ; frame . render_widget (paragraph , area) ; }
};
}
