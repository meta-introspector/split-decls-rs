// Generated macro for impl_60 (impl)
macro_rules! Depcrate_chart_dual_coordimpl_60 {
() => {
// Module: crate::chart::dual_coord
// Provides: {"impl_60"}
// Dependencies: {}
impl < 'a , DB : DrawingBackend , CT1 : CoordTranslate , CT2 : CoordTranslate > DualCoordChartContext < 'a , DB , CT1 , CT2 > { pub (super) fn new (mut primary : ChartContext < 'a , DB , CT1 > , secondary_coord : CT2) -> Self { let secondary_drawing_area = primary . drawing_area . strip_coord_spec () . apply_coord_spec (secondary_coord) ; let mut secondary_x_label_area = [None , None] ; let mut secondary_y_label_area = [None , None] ; std :: mem :: swap (& mut primary . x_label_area [0] , & mut secondary_x_label_area [0]) ; std :: mem :: swap (& mut primary . y_label_area [1] , & mut secondary_y_label_area [1]) ; Self { primary , secondary : ChartContext { x_label_area : secondary_x_label_area , y_label_area : secondary_y_label_area , drawing_area : secondary_drawing_area , series_anno : vec ! [] , drawing_area_pos : (0 , 0) , } , } } # [doc = " Get a reference to the drawing area that uses the secondary coordinate system"] pub fn secondary_plotting_area (& self) -> & DrawingArea < DB , CT2 > { & self . secondary . drawing_area } # [doc = " Borrow a mutable reference to the chart context that uses the secondary"] # [doc = " coordinate system"] pub fn borrow_secondary (& self) -> & ChartContext < 'a , DB , CT2 > { & self . secondary } }
};
}
