macro_rules! deps {
    () => {
        CompassPt!();
        ID!();
    };
}

macro_rules! Port {
    () => {
        deps!();
        # [doc = " This enum corresponds to the `port` non-terminal of the grammar."] # [derive (Debug , Eq , PartialEq , Ord , PartialOrd , Hash , Clone)] pub enum Port { # [doc = " The variant in which an ID is given, and possibly a compass point."] ID (String , Option < CompassPt >) , # [doc = " The variant in which only a compass point is given."] Compass (CompassPt) , }
    };
}

Port!()