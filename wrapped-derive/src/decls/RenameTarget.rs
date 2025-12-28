macro_rules! deps {
    () => {
        Argument!();
        EnumItem!();
    };
}

macro_rules! RenameTarget {
    () => {
        deps!();
        # [derive (Debug , Copy , Clone)] pub enum RenameTarget { Type , EnumItem , Field , Argument , }
    };
}

RenameTarget!();