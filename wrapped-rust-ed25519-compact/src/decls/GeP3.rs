macro_rules! deps {
    () => {
        Fe!();
    };
}

macro_rules! GeP3 {
    () => {
        deps!();
        # [derive (Clone , Copy)] pub struct GeP3 { x : Fe , y : Fe , z : Fe , t : Fe , }
    };
}

GeP3!();