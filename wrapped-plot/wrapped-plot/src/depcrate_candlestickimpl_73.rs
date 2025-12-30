// Generated macro for impl_73 (impl)
macro_rules! Depcrate_candlestickimpl_73 {
() => {
// Module: crate::candlestick
// Provides: {"impl_73"}
// Dependencies: {}
impl < X , WM , BM , BH , WH > traits :: Plot < Candlesticks < X , WM , BM , BH , WH > > for Figure where BH : IntoIterator , BH :: Item : Data , BM : IntoIterator , BM :: Item : Data , WH : IntoIterator , WH :: Item : Data , WM : IntoIterator , WM :: Item : Data , X : IntoIterator , X :: Item : Data , { type Properties = Properties ; fn plot < F > (& mut self , candlesticks : Candlesticks < X , WM , BM , BH , WH > , configure : F ,) -> & mut Figure where F : FnOnce (& mut Properties) -> & mut Properties , { let (x_factor , y_factor) = crate :: scale_factor (& self . axes , crate :: Axes :: BottomXLeftY) ; let Candlesticks { x , whisker_min , box_min , box_high , whisker_high , } = candlesticks ; let data = Matrix :: new (itertools :: izip ! (x , box_min , whisker_min , whisker_high , box_high) , (x_factor , y_factor , y_factor , y_factor , y_factor) ,) ; self . plots . push (Plot :: new (data , configure (& mut Default :: default ()))) ; self } }
};
}
