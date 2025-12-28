macro_rules! deps {
    () => {
        Unit!();
        Location!();
    };
}

macro_rules! Mode {
    () => {
        deps!();
        # [doc = " A way to display a [Unit]."] # [derive (Copy , Clone , Eq , PartialEq , Ord , PartialOrd , Debug , Hash)] pub struct Mode { location : Location , percent : bool , throughput : bool , }
    };
}

Mode!()