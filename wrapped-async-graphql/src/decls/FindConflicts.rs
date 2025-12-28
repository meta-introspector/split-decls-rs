macro_rules! deps {
    () => {
        Field!();
        VisitorContext!();
    };
}

macro_rules! FindConflicts {
    () => {
        deps!();
        struct FindConflicts < 'a , 'ctx > { outputs : HashMap < (Option < & 'a str > , & 'a str) , & 'a Positioned < Field > > , visited : HashSet < & 'a str > , ctx : & 'a mut VisitorContext < 'ctx > , }
    };
}

FindConflicts!();