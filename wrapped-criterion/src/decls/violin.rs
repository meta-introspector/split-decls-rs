macro_rules! deps {
    () => {
        Sample!();
        AxisScale!();
        BenchmarkId!();
        ValueFormatter!();
    };
}

macro_rules! violin {
    () => {
        deps!();
        pub fn violin (formatter : & dyn ValueFormatter , title : & str , all_curves : & [& (& BenchmarkId , Vec < f64 >)] , path : & Path , axis_scale : AxisScale ,) { let all_curves_vec = all_curves . iter () . rev () . cloned () . collect :: < Vec < _ > > () ; let all_curves : & [& (& BenchmarkId , Vec < f64 >)] = & all_curves_vec ; let mut kdes = all_curves . iter () . map (| & & (id , ref sample) | { let (x , mut y) = kde :: sweep (Sample :: new (sample) , KDE_POINTS , None) ; let y_max = Sample :: new (& y) . max () ; for y in y . iter_mut () { * y /= y_max ; } (id . as_title () , x , y) }) . collect :: < Vec < _ > > () ; let mut xs = kdes . iter () . flat_map (| (_ , x , _) | x . iter ()) . filter (| & & x | x > 0.) ; let (mut min , mut max) = { let & first = xs . next () . unwrap () ; (first , first) } ; for & e in xs { if e < min { min = e ; } else if e > max { max = e ; } } let mut dummy = [1.0] ; let unit = formatter . scale_values (max , & mut dummy) ; kdes . iter_mut () . for_each (| & mut (_ , ref mut xs , _) | { formatter . scale_values (max , xs) ; }) ; let mut x_range = plotters :: data :: fitting_range (kdes . iter () . flat_map (| (_ , xs , _) | xs . iter ())) ; x_range . start = 0.0 ; let y_range = - 0.5 .. all_curves . len () as f64 - 0.5 ; let size = (960 , 150 + (18 * all_curves . len () as u32)) ; let root_area = SVGBackend :: new (& path , size) . into_drawing_area () . titled (& format ! ("{}: Violin plot" , title) , (DEFAULT_FONT , 20)) . unwrap () ; match axis_scale { AxisScale :: Linear => draw_violin_figure (root_area , unit , x_range , y_range , kdes) , AxisScale :: Logarithmic => { draw_violin_figure (root_area , unit , x_range . log_scale () , y_range , kdes) ; } } }
    };
}

violin!()