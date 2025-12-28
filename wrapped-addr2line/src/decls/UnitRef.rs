macro_rules! deps {
    () => {
        DebugFile!();
    };
}

macro_rules! UnitRef {
    () => {
        deps!();
        type UnitRef < 'unit , R > = (DebugFile , gimli :: UnitRef < 'unit , R >) ;
    };
}

UnitRef!()