// Generated macro for App (struct)
macro_rules! Depcrate_appApp {
() => {
// Module: crate::app
// Provides: {"App"}
// Dependencies: {}
pub struct App < 'a > { pub title : & 'a str , pub should_quit : bool , pub tabs : TabsState < 'a > , pub show_chart : bool , pub progress : f64 , pub sparkline : Signal < RandomSignal > , pub tasks : StatefulList < & 'a str > , pub logs : StatefulList < (& 'a str , & 'a str) > , pub signals : Signals , pub barchart : Vec < (& 'a str , u64) > , pub servers : Vec < Server < 'a > > , pub enhanced_graphics : bool , }
};
}
