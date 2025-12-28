macro_rules! deps {
    () => {
        UseTreeKind!();
    };
}

macro_rules! UseTree {
    () => {
        deps!();
        # [derive (Debug , Clone , Eq , PartialEq)] pub struct UseTree { kind : UseTreeKind , }
    };
}

UseTree!()