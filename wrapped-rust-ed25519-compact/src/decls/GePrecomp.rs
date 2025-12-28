macro_rules! deps {
    () => {
        Fe!();
    };
}

macro_rules! GePrecomp {
    () => {
        deps!();
        # [derive (Clone , Copy)] pub struct GePrecomp { y_plus_x : Fe , y_minus_x : Fe , xy2d : Fe , }
    };
}

GePrecomp!()