macro_rules! deps {
    () => {
        Recorder!();
        Location!();
        Visit!();
        Entry!();
    };
}

macro_rules! impl_47 {
    () => {
        deps!();
        impl Visit for Recorder { fn pop_back_tracked_path_and_set_current (& mut self) { if let Some (Location :: Path) = self . location { self . path = self . path_deque . pop_back () . unwrap_or_default () ; } } fn pop_front_tracked_path_and_set_current (& mut self) { if let Some (Location :: Path) = self . location { self . path = self . path_deque . pop_front () . expect ("every call is matched with push_tracked_path_component") ; } } fn push_back_tracked_path_component (& mut self , component : & BStr) { if let Some (Location :: Path) = self . location { self . push_element (component) ; self . path_deque . push_back (self . path . clone ()) ; } } fn push_path_component (& mut self , component : & BStr) { match self . location { None => { } Some (Location :: Path) => { self . push_element (component) ; } Some (Location :: FileName) => { self . path . clear () ; self . path . extend_from_slice (component) ; } } } fn pop_path_component (& mut self) { if let Some (Location :: Path) = self . location { self . pop_element () ; } } fn visit_tree (& mut self , entry : & tree :: EntryRef < '_ >) -> Action { self . records . push (Entry :: new (entry , self . path_clone ())) ; Action :: Continue } fn visit_nontree (& mut self , entry : & tree :: EntryRef < '_ >) -> Action { self . records . push (Entry :: new (entry , self . path_clone ())) ; Action :: Continue } }
    };
}

impl_47!()