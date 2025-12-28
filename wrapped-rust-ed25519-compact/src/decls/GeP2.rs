macro_rules! deps {
    () => {
        Fe!();
    };
}

macro_rules! GeP2 {
    () => {
        deps!();
        # [derive (Clone , Copy)] pub struct GeP2 { x : Fe , y : Fe , z : Fe , }
    };
}

GeP2!();