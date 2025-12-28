macro_rules! deps {
    () => {
        Delegate!();
        Visit!();
        Change!();
        ChangeRef!();
        Error!();
        Action!();
    };
}

macro_rules! impl_74 {
    () => {
        deps!();
        impl < VisitFn , E , Objects > crate :: tree :: Visit for Delegate < '_ , '_ , VisitFn , E , Objects > where Objects : gix_object :: FindObjectOrHeader , VisitFn : for < 'delegate > FnMut (ChangeRef < '_ >) -> Result < Action , E > , E : Into < Box < dyn std :: error :: Error + Sync + Send + 'static > > , { fn pop_front_tracked_path_and_set_current (& mut self) { self . recorder . pop_front_tracked_path_and_set_current () ; } fn push_back_tracked_path_component (& mut self , component : & BStr) { self . recorder . push_back_tracked_path_component (component) ; } fn push_path_component (& mut self , component : & BStr) { self . recorder . push_path_component (component) ; } fn pop_path_component (& mut self) { self . recorder . pop_path_component () ; } fn visit (& mut self , change : crate :: tree :: visit :: Change) -> crate :: tree :: visit :: Action { match self . tracked . as_mut () { Some (tracked) => tracked . try_push_change (change , self . recorder . path ()) . map_or (crate :: tree :: visit :: Action :: Continue , | change | { Self :: emit_change (change , self . recorder . path () , & mut self . visit , & mut self . err) }) , None => Self :: emit_change (change , self . recorder . path () , & mut self . visit , & mut self . err) , } } }
    };
}

impl_74!()