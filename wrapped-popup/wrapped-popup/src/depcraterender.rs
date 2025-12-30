// Generated macro for render (function)
macro_rules! Depcraterender {
() => {
// Module: crate
// Provides: {"render"}
// Dependencies: {}
fn render (frame : & mut Frame , show_popup : bool) { let area = frame . area () ; let layout = Layout :: vertical ([Constraint :: Length (1) , Constraint :: Fill (1)]) ; let [instructions , content] = area . layout (& layout) ; frame . render_widget (Line :: from ("Press 'p' to toggle popup, 'q' to quit") . centered () , instructions ,) ; frame . render_widget (Block :: bordered () . title ("Content") . on_blue () , content) ; if show_popup { let popup = Block :: bordered () . title ("Popup") ; let popup_area = centered_area (area , 60 , 20) ; frame . render_widget (Clear , popup_area) ; frame . render_widget (popup , popup_area) ; } }
};
}
