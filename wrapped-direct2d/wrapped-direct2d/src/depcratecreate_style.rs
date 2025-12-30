// Generated macro for create_style (function)
macro_rules! Depcratecreate_style {
() => {
// Module: crate
// Provides: {"create_style"}
// Dependencies: {}
fn create_style (factory : & ID2D1Factory1) -> Result < ID2D1StrokeStyle1 > { let props = D2D1_STROKE_STYLE_PROPERTIES1 { startCap : D2D1_CAP_STYLE_ROUND , endCap : D2D1_CAP_STYLE_TRIANGLE , .. Default :: default () } ; unsafe { factory . CreateStrokeStyle (& props , None) } }
};
}
