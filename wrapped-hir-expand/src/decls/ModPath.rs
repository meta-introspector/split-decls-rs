macro_rules! deps {
    () => {
        Name!();
        PathKind!();
    };
}

macro_rules! ModPath {
    () => {
        deps!();
        # [derive (Debug , Clone , PartialEq , Eq , PartialOrd , Ord , Hash)] pub struct ModPath { pub kind : PathKind , segments : SmallVec < Name , 1 > , }
    };
}

ModPath!();