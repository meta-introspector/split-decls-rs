// Generated macro for pdf_comparison (function)
macro_rules! Depcrate_plot_gnuplot_backend_pdfpdf_comparison {
() => {
// Module: crate::plot::gnuplot_backend::pdf
// Provides: {"pdf_comparison"}
// Dependencies: {}
pub (crate) fn pdf_comparison (id : & BenchmarkId , context : & ReportContext , formatter : & dyn ValueFormatter , measurements : & MeasurementData < '_ > , comparison : & ComparisonData , size : Option < Size > ,) -> Child { let mut figure = pdf_comparison_figure (formatter , measurements , comparison , size) ; figure . set (Title (gnuplot_escape (id . as_title ()))) ; let path = context . report_path (id , "both/pdf.svg") ; debug_script (& path , & figure) ; figure . set (Output (path)) . draw () . unwrap () }
};
}
