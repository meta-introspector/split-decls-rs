// Generated macro for impl_34 (impl)
macro_rules! Depcrateimpl_34 {
() => {
// Module: crate
// Provides: {"impl_34"}
// Dependencies: {}
impl Example { fn illustration (constraint : Constraint , width : u16) -> impl Widget { let color = match constraint { Constraint :: Length (_) => LENGTH_COLOR , Constraint :: Percentage (_) => PERCENTAGE_COLOR , Constraint :: Ratio (_ , _) => RATIO_COLOR , Constraint :: Fill (_) => FILL_COLOR , Constraint :: Min (_) => MIN_COLOR , Constraint :: Max (_) => MAX_COLOR , } ; let fg = Color :: White ; let title = format ! ("{constraint}") ; let content = format ! ("{width} px") ; let text = format ! ("{title}\n{content}") ; let block = Block :: bordered () . border_set (symbols :: border :: QUADRANT_OUTSIDE) . border_style (Style :: reset () . fg (color) . reversed ()) . style (Style :: default () . fg (fg) . bg (color)) ; Paragraph :: new (text) . centered () . block (block) } }
};
}
