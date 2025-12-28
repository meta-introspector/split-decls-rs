macro_rules! deps {
    () => {
        Positioned!();
        Directive!();
        ConstDirective!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        impl ConstDirective { # [doc = " Convert this `ConstDirective` into a `Directive`."] # [must_use] pub fn into_directive (self) -> Directive { Directive { name : self . name , arguments : self . arguments . into_iter () . map (| (name , value) | (name , value . map (ConstValue :: into_value))) . collect () , } } # [doc = " Get the argument with the given name."] # [must_use] pub fn get_argument (& self , name : & str) -> Option < & Positioned < ConstValue > > { self . arguments . iter () . find (| item | item . 0 . node == name) . map (| item | & item . 1) } }
    };
}

impl_45!();