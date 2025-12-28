macro_rules! deps {
    () => {
        NothingPrint!();
    };
}

macro_rules! ViaNothing {
    () => {
        deps!();
        # [doc (hidden)] pub trait ViaNothing { fn debug_string (& self) -> NothingPrint ; }
    };
}

ViaNothing!();