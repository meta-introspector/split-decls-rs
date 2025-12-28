macro_rules! deps {
    () => {
        RegionElement!();
    };
}

macro_rules! pretty_print_points {
    () => {
        deps!();
        # [doc = " For debugging purposes, returns a pretty-printed string of the given points."] pub (crate) fn pretty_print_points (location_map : & DenseLocationMap , points : impl IntoIterator < Item = PointIndex > ,) -> String { pretty_print_region_elements (points . into_iter () . take_while (| & p | location_map . point_in_range (p)) . map (| p | location_map . to_location (p)) . map (RegionElement :: Location) ,) }
    };
}

pretty_print_points!();