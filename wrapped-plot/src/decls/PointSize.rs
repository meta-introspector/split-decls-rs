macro_rules! deps {
    () => {
        Size!();
    };
}

macro_rules! PointSize {
    () => {
        deps!();
        # [doc = " Size of the points"] # [derive (Clone , Copy)] pub struct PointSize (pub f64) ;
    };
}

PointSize!()