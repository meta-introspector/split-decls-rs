macro_rules! deps {
    () => {
        Recorder!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        impl Recorder { fn pop_element (& mut self) { if let Some (pos) = self . path . rfind_byte (b'/') { self . path . resize (pos , 0) ; } else { self . path . clear () ; } } fn push_element (& mut self , name : & BStr) { if name . is_empty () { return ; } if ! self . path . is_empty () { self . path . push (b'/') ; } self . path . push_str (name) ; } }
    };
}

impl_56!();