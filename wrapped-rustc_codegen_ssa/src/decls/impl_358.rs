macro_rules! deps {
    () => {
        CopyPath!();
        DebugArgPath!();
    };
}

macro_rules! impl_358 {
    () => {
        deps!();
        impl < 'a > CopyPath < 'a > { pub fn new (from : & 'a Path , to : & 'a Path , error : Error) -> CopyPath < 'a > { CopyPath { from : DebugArgPath (from) , to : DebugArgPath (to) , error } } }
    };
}

impl_358!();