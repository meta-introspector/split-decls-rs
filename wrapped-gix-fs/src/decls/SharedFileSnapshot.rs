macro_rules! deps {
    () => {
        FileSnapshot!();
    };
}

macro_rules! SharedFileSnapshot {
    () => {
        deps!();
        # [doc = " A snapshot of a resource which is up-to-date in the moment it is retrieved."] pub type SharedFileSnapshot < T > = OwnShared < FileSnapshot < T > > ;
    };
}

SharedFileSnapshot!()