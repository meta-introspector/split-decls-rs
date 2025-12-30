// Generated macro for test (module)
macro_rules! Depcrate_element_boxplottest {
() => {
// Module: crate::element::boxplot
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; use crate :: prelude :: * ; # [test] fn test_draw_v () { let root = MockedBackend :: new (1024 , 768) . into_drawing_area () ; let chart = ChartBuilder :: on (& root) . build_cartesian_2d (0 .. 2 , 0f32 .. 100f32) . unwrap () ; let values = Quartiles :: new (& [6]) ; assert ! (chart . plotting_area () . draw (& Boxplot :: new_vertical (1 , & values)) . is_ok ()) ; } # [test] fn test_draw_h () { let root = MockedBackend :: new (1024 , 768) . into_drawing_area () ; let chart = ChartBuilder :: on (& root) . build_cartesian_2d (0f32 .. 100f32 , 0 .. 2) . unwrap () ; let values = Quartiles :: new (& [6]) ; assert ! (chart . plotting_area () . draw (& Boxplot :: new_horizontal (1 , & values)) . is_ok ()) ; } }
};
}
