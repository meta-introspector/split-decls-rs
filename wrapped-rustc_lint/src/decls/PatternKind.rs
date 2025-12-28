macro_rules! PatternKind {
    () => {
        # [derive (Debug , Clone , Copy , PartialEq , Eq)] enum PatternKind { Borrow { mutbl : Mutability } , Assign , }
    };
}

PatternKind!();