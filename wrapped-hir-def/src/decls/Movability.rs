macro_rules! deps {
    () => {
        Static!();
    };
}

macro_rules! Movability {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub enum Movability { Static , Movable , }
    };
}

Movability!()