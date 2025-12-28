macro_rules! deps {
    () => {
        ValueType!();
    };
}

macro_rules! draw_line_comarision_figure {
    () => {
        deps!();
        fn draw_line_comarision_figure < XR : AsRangedCoord < Value = f64 > , YR : AsRangedCoord < Value = f64 > > (root_area : DrawingArea < SVGBackend , Shift > , y_unit : & str , x_range : XR , y_range : YR , value_type : ValueType , data : Vec < (Option < & String > , Vec < f64 > , Vec < f64 >) > ,) where XR :: CoordDescType : PlottersValueFormatter < f64 > , YR :: CoordDescType : PlottersValueFormatter < f64 > , { let input_suffix = match value_type { ValueType :: Bytes => " Size (Bytes)" , ValueType :: Elements => " Size (Elements)" , ValueType :: Bits => " Size (Bits)" , ValueType :: Value => "" , } ; let mut chart = ChartBuilder :: on (& root_area) . margin ((5) . percent ()) . set_label_area_size (LabelAreaPosition :: Left , (5) . percent_width () . min (60)) . set_label_area_size (LabelAreaPosition :: Bottom , (5) . percent_height () . min (40)) . build_cartesian_2d (x_range , y_range) . unwrap () ; chart . configure_mesh () . disable_mesh () . x_desc (format ! ("Input{}" , input_suffix)) . y_desc (format ! ("Average time ({})" , y_unit)) . draw () . unwrap () ; for (id , (name , xs , ys)) in (0 ..) . zip (data) { let series = chart . draw_series (LineSeries :: new (xs . into_iter () . zip (ys) , COMPARISON_COLORS [id % NUM_COLORS] . filled () ,) . point_size (POINT_SIZE) ,) . unwrap () ; if let Some (name) = name { series . label (name) . legend (move | (x , y) | { Rectangle :: new ([(x , y - 5) , (x + 20 , y + 5)] , COMPARISON_COLORS [id % NUM_COLORS] . filled () ,) }) ; } } chart . configure_series_labels () . position (SeriesLabelPosition :: UpperLeft) . draw () . unwrap () ; }
    };
}

draw_line_comarision_figure!()