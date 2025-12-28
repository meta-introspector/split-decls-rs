macro_rules! deps {
    () => {
        LocalId!();
        Place!();
        Rvalue!();
    };
}

macro_rules! StatementKind {
    () => {
        deps!();
        # [derive (Debug , PartialEq , Eq , Clone)] pub enum StatementKind < 'db > { Assign (Place < 'db > , Rvalue < 'db >) , FakeRead (Place < 'db >) , Deinit (Place < 'db >) , StorageLive (LocalId < 'db >) , StorageDead (LocalId < 'db >) , Nop , }
    };
}

StatementKind!();