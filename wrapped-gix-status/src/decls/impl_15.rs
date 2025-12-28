macro_rules! deps {
    () => {
        Entry!();
        SubmoduleStatus!();
        Recorder!();
        Record!();
        VisitEntry!();
        EntryStatus!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl < 'index , T : Send , U : Send > VisitEntry < 'index > for Recorder < 'index , T , U > { type ContentChange = T ; type SubmoduleStatus = U ; fn visit_entry (& mut self , _entries : & 'index [index :: Entry] , entry : & 'index index :: Entry , entry_index : usize , relative_path : & 'index BStr , status : EntryStatus < Self :: ContentChange , Self :: SubmoduleStatus > ,) { self . records . push (Record { entry , entry_index , relative_path , status , }) ; } }
    };
}

impl_15!();