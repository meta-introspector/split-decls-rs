macro_rules! deps {
    () => {
        Entry!();
        SubmoduleStatus!();
        Recorder!();
        VisitEntry!();
    };
}

macro_rules! impl_53 {
    () => {
        deps!();
        impl < 'index , T : Send , U : Send > VisitEntry < 'index > for Recorder < 'index , T , U > { type ContentChange = T ; type SubmoduleStatus = U ; fn visit_entry (& mut self , entry : Entry < 'index , Self :: ContentChange , Self :: SubmoduleStatus >) { self . records . push (entry) ; } }
    };
}

impl_53!();