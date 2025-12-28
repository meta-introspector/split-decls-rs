macro_rules! deps {
    () => {
        Comparison!();
        AxisScale!();
        ValueFormatter!();
        ValueType!();
        BenchmarkId!();
    };
}

macro_rules! line_comparison {
    () => {
        deps!();
        pub fn line_comparison (formatter : & dyn ValueFormatter , title : & str , all_curves : & [& (& BenchmarkId , Vec < f64 >)] , path : & Path , value_type : ValueType , axis_scale : AxisScale ,) { let (unit , series_data) = line_comparison_series_data (formatter , all_curves) ; let x_range = plotters :: data :: fitting_range (series_data . iter () . flat_map (| (_ , xs , _) | xs . iter ())) ; let y_range = plotters :: data :: fitting_range (series_data . iter () . flat_map (| (_ , _ , ys) | ys . iter ())) ; let root_area = SVGBackend :: new (& path , SIZE) . into_drawing_area () . titled (& format ! ("{}: Comparison" , title) , (DEFAULT_FONT , 20)) . unwrap () ; match axis_scale { AxisScale :: Linear => { draw_line_comarision_figure (root_area , unit , x_range , y_range , value_type , series_data) ; } AxisScale :: Logarithmic => draw_line_comarision_figure (root_area , unit , x_range . log_scale () , y_range . log_scale () , value_type , series_data ,) , } }
    };
}

line_comparison!();