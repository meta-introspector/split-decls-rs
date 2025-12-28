macro_rules! deps {
    () => {
        Fe!();
    };
}

macro_rules! GeP1P1 {
    () => {
        deps!();
        # [derive (Clone , Copy , Default)] pub struct GeP1P1 { x : Fe , y : Fe , z : Fe , t : Fe , }
    };
}

GeP1P1!()