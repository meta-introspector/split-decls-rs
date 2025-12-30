// Generated macro for iteration_times_figure (function)
macro_rules! Depcrate_plot_gnuplot_backend_iteration_timesiteration_times_figure {
() => {
// Module: crate::plot::gnuplot_backend::iteration_times
// Provides: {"iteration_times_figure"}
// Dependencies: {}
fn iteration_times_figure (formatter : & dyn ValueFormatter , measurements : & MeasurementData < '_ > , size : Option < Size > ,) -> Figure { let data = & measurements . avg_times ; let max_avg_time = data . max () ; let mut scaled_y : Vec < _ > = data . iter () . map (| (f , _) | f) . collect () ; let unit = formatter . scale_values (max_avg_time , & mut scaled_y) ; let scaled_y = Sample :: new (& scaled_y) ; let mut figure = Figure :: new () ; figure . set (Font (DEFAULT_FONT)) . set (size . unwrap_or (SIZE)) . configure (Axis :: BottomX , | a | { a . configure (Grid :: Major , | g | g . show ()) . set (Label ("Sample")) }) . configure (Axis :: LeftY , | a | { a . configure (Grid :: Major , | g | g . show ()) . set (Label (format ! ("Average Iteration Time ({})" , unit))) }) . plot (Points { x : 1 .. (data . len () + 1) , y : scaled_y . as_ref () , } , | c | { c . set (DARK_BLUE) . set (PointSize (0.5)) . set (PointType :: FilledCircle) } ,) ; figure }
};
}
