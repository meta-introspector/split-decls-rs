macro_rules! deps {
    () => {
        Kind!();
        Mode!();
    };
}

macro_rules! Unit {
    () => {
        deps!();
        # [doc = " A configurable and flexible unit for use in [Progress::init()][crate::Progress::init()]."] # [derive (Debug , Clone , Hash)] pub struct Unit { kind : Kind , mode : Option < display :: Mode > , }
    };
}

Unit!()