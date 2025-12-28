macro_rules! Data {
    () => {
        # [doc = " Types that can be plotted"] pub trait Data { # [doc = " Convert the type into a double precision float"] fn f64 (self) -> f64 ; }
    };
}

Data!();