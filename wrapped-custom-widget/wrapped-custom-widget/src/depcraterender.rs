// Generated macro for render (function)
macro_rules! Depcraterender {
() => {
// Module: crate
// Provides: {"render"}
// Dependencies: {}
fn render (frame : & mut Frame , states : [State ; 3]) { let layout = Layout :: vertical ([Constraint :: Length (1) , Constraint :: Max (3) , Constraint :: Length (1) , Constraint :: Min (0) ,]) ; let [title , buttons , help , _] = frame . area () . layout (& layout) ; frame . render_widget (Paragraph :: new ("Custom Widget Example (mouse enabled)") , title ,) ; render_buttons (frame , buttons , states) ; frame . render_widget (Paragraph :: new ("←/→: select, Space: toggle, q: quit") , help) ; }
};
}
