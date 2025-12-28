macro_rules! deps {
    () => {
        TypeMap!();
        Warnings!();
        Config!();
        WarningBuilder!();
    };
}

macro_rules! impl_328 {
    () => {
        deps!();
        impl WarningBuilder { pub fn build (self) -> Warnings { Warnings (self . 0 . write () . unwrap () . split_off (0)) } pub fn add (& self , message : String) { self . 0 . write () . unwrap () . push (message) ; } pub fn skip_method (& self , method : MethodDef , dependencies : & TypeMap , config : & Config) { let mut message = String :: new () ; writeln ! (& mut message , "skipping `{}.{}` due to missing dependencies:" , method . parent () . type_name () , method . name ()) . unwrap () ; for tn in dependencies . keys () { if ! config . types . contains_key (tn) && config . references . contains (* tn) . is_none () { writeln ! (& mut message , "  {tn}") . unwrap () ; } } self . add (message) ; } }
    };
}

impl_328!()