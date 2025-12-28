macro_rules! deps {
    () => {
        Sleep!();
    };
}

macro_rules! impl_183 {
    () => {
        deps!();
        impl dyn Sleep { # ! [doc = " This is a re-implementation of downcast methods from std::any::Any"] # [doc = " Check whether the type is the same as `T`"] pub fn is < T > (& self) -> bool where T : Sleep + 'static , { self . __type_id (private :: Sealed { }) == TypeId :: of :: < T > () } # [doc = " Downcast a pinned &mut Sleep object to its original type"] pub fn downcast_mut_pin < T > (self : Pin < & mut Self >) -> Option < Pin < & mut T > > where T : Sleep + 'static , { if self . is :: < T > () { unsafe { let inner = Pin :: into_inner_unchecked (self) ; Some (Pin :: new_unchecked (& mut * (& mut * inner as * mut dyn Sleep as * mut T) ,)) } } else { None } } }
    };
}

impl_183!()