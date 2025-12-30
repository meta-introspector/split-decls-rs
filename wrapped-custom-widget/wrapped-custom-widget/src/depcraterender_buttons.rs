// Generated macro for render_buttons (function)
macro_rules! Depcraterender_buttons {
() => {
// Module: crate
// Provides: {"render_buttons"}
// Dependencies: {}
fn render_buttons (frame : & mut Frame < '_ > , area : Rect , states : [State ; 3]) { let layout = Layout :: horizontal ([Constraint :: Length (15) ; 3]) . flex (Flex :: Start) ; let [red , green , blue] = area . layout (& layout) ; frame . render_widget (Button :: new ("Red") . theme (RED) . state (states [0]) , red) ; frame . render_widget (Button :: new ("Green") . theme (GREEN) . state (states [1]) , green) ; frame . render_widget (Button :: new ("Blue") . theme (BLUE) . state (states [2]) , blue) ; }
};
}
