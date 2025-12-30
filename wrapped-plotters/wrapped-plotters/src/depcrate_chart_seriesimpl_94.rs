// Generated macro for impl_94 (impl)
macro_rules! Depcrate_chart_seriesimpl_94 {
() => {
// Module: crate::chart::series
// Provides: {"impl_94"}
// Dependencies: {}
impl SeriesLabelPosition { fn layout_label_area (& self , label_dim : (i32 , i32) , area_dim : (u32 , u32)) -> (i32 , i32) { use SeriesLabelPosition :: * ; (match self { UpperLeft | MiddleLeft | LowerLeft => 5 , UpperMiddle | MiddleMiddle | LowerMiddle => (area_dim . 0 as i32 - label_dim . 0) / 2 , UpperRight | MiddleRight | LowerRight => area_dim . 0 as i32 - label_dim . 0 - 5 , Coordinate (x , _) => * x , } , match self { UpperLeft | UpperMiddle | UpperRight => 5 , MiddleLeft | MiddleMiddle | MiddleRight => (area_dim . 1 as i32 - label_dim . 1) / 2 , LowerLeft | LowerMiddle | LowerRight => area_dim . 1 as i32 - label_dim . 1 - 5 , Coordinate (_ , y) => * y , } ,) } }
};
}
