macro_rules! deps {
    () => {
        Recorder!();
        Action!();
        Visit!();
        Change!();
        Location!();
    };
}

macro_rules! impl_57 {
    () => {
        deps!();
        impl Visit for Recorder { fn pop_front_tracked_path_and_set_current (& mut self) { if let Some (Location :: Path) = self . location { self . path = self . path_deque . pop_front () . expect ("every parent is set only once") ; } } fn push_back_tracked_path_component (& mut self , component : & BStr) { match self . location { None => { } Some (Location :: Path) => { self . push_element (component) ; self . path_deque . push_back (self . path . clone ()) ; } Some (Location :: FileName) => { self . path . clear () ; self . path . extend_from_slice (component) ; } } } fn push_path_component (& mut self , component : & BStr) { match self . location { None => { } Some (Location :: Path) => { self . push_element (component) ; } Some (Location :: FileName) => { self . path . clear () ; self . path . extend_from_slice (component) ; } } } fn pop_path_component (& mut self) { if let Some (Location :: Path) = self . location { self . pop_element () ; } } fn visit (& mut self , change : visit :: Change) -> visit :: Action { use visit :: Change :: * ; self . records . push (match change { Deletion { entry_mode , oid , relation , } => Change :: Deletion { entry_mode , oid , path : self . path_clone () , relation , } , Addition { entry_mode , oid , relation , } => Change :: Addition { entry_mode , oid , path : self . path_clone () , relation , } , Modification { previous_entry_mode , previous_oid , entry_mode , oid , } => Change :: Modification { previous_entry_mode , previous_oid , entry_mode , oid , path : self . path_clone () , } , }) ; visit :: Action :: Continue } }
    };
}

impl_57!()