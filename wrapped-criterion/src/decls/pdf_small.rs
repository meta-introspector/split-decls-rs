macro_rules! deps {
    () => {
        Sample!();
        MeasurementData!();
        BenchmarkId!();
        ValueFormatter!();
        ReportContext!();
    };
}

macro_rules! pdf_small {
    () => {
        deps!();
        pub (crate) fn pdf_small (id : & BenchmarkId , context : & ReportContext , formatter : & dyn ValueFormatter , measurements : & MeasurementData < '_ > , size : Option < (u32 , u32) > ,) { let avg_times = & * measurements . avg_times ; let typical = avg_times . max () ; let mut scaled_avg_times : Vec < f64 > = (avg_times as & Sample < f64 >) . iter () . cloned () . collect () ; let unit = formatter . scale_values (typical , & mut scaled_avg_times) ; let scaled_avg_times = Sample :: new (& scaled_avg_times) ; let mean = scaled_avg_times . mean () ; let (xs , ys , mean_y) = kde :: sweep_and_estimate (scaled_avg_times , KDE_POINTS , None , mean) ; let xs_ = Sample :: new (& xs) ; let ys_ = Sample :: new (& ys) ; let y_limit = ys_ . max () * 1.1 ; let path = context . report_path (id , "pdf_small.svg") ; let size = size . unwrap_or (SIZE) ; let root_area = SVGBackend :: new (& path , (size . 0 , size . 1)) . into_drawing_area () ; let mut chart = ChartBuilder :: on (& root_area) . margin ((5) . percent ()) . set_label_area_size (LabelAreaPosition :: Left , (5) . percent_width () . min (60)) . set_label_area_size (LabelAreaPosition :: Bottom , (5) . percent_height () . min (40)) . build_cartesian_2d (xs_ . min () .. xs_ . max () , 0.0 .. y_limit) . unwrap () ; chart . configure_mesh () . disable_mesh () . y_desc ("Density (a.u.)") . x_desc (format ! ("Average Time ({})" , unit)) . x_label_formatter (& | & x | pretty_print_float (x , true)) . y_label_formatter (& | & y | pretty_print_float (y , true)) . x_labels (5) . draw () . unwrap () ; chart . draw_series (AreaSeries :: new (xs . iter () . zip (ys . iter ()) . map (| (x , y) | (* x , * y)) , 0.0 , DARK_BLUE . mix (0.25) . filled () ,)) . unwrap () ; chart . draw_series (std :: iter :: once (PathElement :: new (vec ! [(mean , 0.0) , (mean , mean_y)] , DARK_BLUE . filled () . stroke_width (2) ,))) . unwrap () ; }
    };
}

pdf_small!()