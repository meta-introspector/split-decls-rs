macro_rules! deps {
    () => {
        LineInstructions!();
        Reader!();
        Result!();
    };
}

macro_rules! impl_415 {
    () => {
        deps!();
        impl < R : Reader > LineInstructions < R > { fn remove_trailing (& self , other : & LineInstructions < R >) -> Result < LineInstructions < R > > { let offset = other . input . offset_from (& self . input) ; let mut input = self . input . clone () ; input . truncate (offset) ? ; Ok (LineInstructions { input }) } }
    };
}

impl_415!();