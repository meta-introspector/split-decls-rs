macro_rules! deps {
    () => {
        Fe!();
    };
}

macro_rules! GeCached {
    () => {
        deps!();
        # [derive (Clone , Copy , Default)] pub struct GeCached { y_plus_x : Fe , y_minus_x : Fe , z : Fe , t2d : Fe , }
    };
}

GeCached!()