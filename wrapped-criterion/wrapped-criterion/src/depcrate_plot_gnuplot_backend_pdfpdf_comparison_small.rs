// Generated macro for pdf_comparison_small (function)
macro_rules! Depcrate_plot_gnuplot_backend_pdfpdf_comparison_small {
() => {
// Module: crate::plot::gnuplot_backend::pdf
// Provides: {"pdf_comparison_small"}
// Dependencies: {}
pub (crate) fn pdf_comparison_small (id : & BenchmarkId , context : & ReportContext , formatter : & dyn ValueFormatter , measurements : & MeasurementData < '_ > , comparison : & ComparisonData , size : Option < Size > ,) -> Child { let mut figure = pdf_comparison_figure (formatter , measurements , comparison , size) ; figure . configure (Key , | k | k . hide ()) ; let path = context . report_path (id , "relative_pdf_small.svg") ; debug_script (& path , & figure) ; figure . set (Output (path)) . draw () . unwrap () }
};
}
