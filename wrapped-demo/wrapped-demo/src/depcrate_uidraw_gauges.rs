// Generated macro for draw_gauges (function)
macro_rules! Depcrate_uidraw_gauges {
() => {
// Module: crate::ui
// Provides: {"draw_gauges"}
// Dependencies: {}
fn draw_gauges (frame : & mut Frame , app : & mut App , area : Rect) { let chunks = Layout :: vertical ([Constraint :: Length (2) , Constraint :: Length (3) , Constraint :: Length (2) ,]) . margin (1) . split (area) ; let block = Block :: bordered () . title ("Graphs") ; frame . render_widget (block , area) ; let label = format ! ("{:.2}%" , app . progress * 100.0) ; let gauge = Gauge :: default () . block (Block :: new () . title ("Gauge:")) . gauge_style (Style :: default () . fg (Color :: Magenta) . bg (Color :: Black) . add_modifier (Modifier :: ITALIC | Modifier :: BOLD) ,) . use_unicode (app . enhanced_graphics) . label (label) . ratio (app . progress) ; frame . render_widget (gauge , chunks [0]) ; let sparkline = Sparkline :: default () . block (Block :: new () . title ("Sparkline:")) . style (Style :: default () . fg (Color :: Green)) . data (& app . sparkline . points) . bar_set (if app . enhanced_graphics { symbols :: bar :: NINE_LEVELS } else { symbols :: bar :: THREE_LEVELS }) ; frame . render_widget (sparkline , chunks [1]) ; let line_gauge = LineGauge :: default () . block (Block :: new () . title ("LineGauge:")) . filled_style (Style :: default () . fg (Color :: Magenta)) . filled_symbol (if app . enhanced_graphics { symbols :: line :: THICK_HORIZONTAL } else { symbols :: line :: HORIZONTAL }) . unfilled_symbol (if app . enhanced_graphics { symbols :: line :: THICK_HORIZONTAL } else { symbols :: line :: HORIZONTAL }) . ratio (app . progress) ; frame . render_widget (line_gauge , chunks [2]) ; }
};
}
