macro_rules! deps {
    () => {
        Entry!();
        VisitEntry!();
    };
}

macro_rules! Recorder {
    () => {
        deps!();
        # [doc = " Convenience implementation of [`VisitEntry`] that collects all changes into a `Vec`."] # [derive (Debug , Default)] pub struct Recorder < 'index , T = () , U = () > { # [doc = " The collected changes."] pub records : Vec < Entry < 'index , T , U > > , }
    };
}

Recorder!()