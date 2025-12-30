// Generated macro for impl_92 (impl)
macro_rules! Depcrate_chart_seriesimpl_92 {
() => {
// Module: crate::chart::series
// Provides: {"impl_92"}
// Dependencies: {}
impl < 'a , DB : DrawingBackend > SeriesAnno < 'a , DB > { # [allow (clippy :: option_as_ref_deref)] pub (crate) fn get_label (& self) -> & str { self . label . as_ref () . map (| x | x . as_str ()) . unwrap_or ("") } pub (crate) fn get_draw_func (& self) -> Option < & SeriesAnnoDrawFn < 'a , DB > > { self . draw_func . as_ref () . map (| x | x . as_ref ()) } pub (crate) fn new () -> Self { Self { label : None , draw_func : None , } } # [doc = "\n    Sets the series label for the current series.\n\n    See [`ChartContext::configure_series_labels()`] for more information and examples.\n    "] pub fn label < L : Into < String > > (& mut self , label : L) -> & mut Self { self . label = Some (label . into ()) ; self } # [doc = "\n    Sets the legend element creator function.\n\n    - `func`: The function use to create the element\n\n    # Note\n\n    The creation function uses a shifted pixel-based coordinate system, where the\n    point (0,0) is defined to the mid-right point of the shape.\n\n    # See also\n\n    See [`ChartContext::configure_series_labels()`] for more information and examples.\n    "] pub fn legend < E : IntoDynElement < 'a , DB , BackendCoord > , T : Fn (BackendCoord) -> E + 'a > (& mut self , func : T ,) -> & mut Self { self . draw_func = Some (Box :: new (move | p | func (p) . into_dyn ())) ; self } }
};
}
