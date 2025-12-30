// Generated macro for render_logo (function)
macro_rules! Depcraterender_logo {
() => {
// Module: crate
// Provides: {"render_logo"}
// Dependencies: {}
fn render_logo (frame : & mut Frame , area : Rect) { let area = area . inner (Margin :: new (1 , 0)) ; let layout = Layout :: vertical (Constraint :: from_lengths ([6 , 2 , 1])) . flex (Flex :: End) ; let [shadow_area , logo_area , version_area] = area . layout (& layout) ; let letter_layout = Layout :: horizontal (Constraint :: from_lengths ([5 , 4 , 4 , 4 , 4 , 5 , 1])) ; for (row_index , row) in shadow_area . rows () . enumerate () { for (rainbow , letter_area) in zip (Rainbow :: ROYGBIV , row . layout_vec (& letter_layout)) { let color = rainbow . gradient_color (row_index) ; frame . render_widget (Block :: new () . style (color) , letter_area) ; } frame . render_widget (RatatuiLogo :: small () , row) ; } frame . render_widget (Block :: new () . style (FG_COLOR) , logo_area) ; frame . render_widget (RatatuiLogo :: small () , logo_area) ; frame . render_widget (format ! ("v{SEMVER} \"{RELEASE_NAME}\"") . dim () , version_area) ; }
};
}
