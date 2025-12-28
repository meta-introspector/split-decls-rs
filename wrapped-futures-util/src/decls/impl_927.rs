macro_rules! impl_927 {
    () => {
        impl < Si1 : Debug , Si2 : Debug > Debug for Fanout < Si1 , Si2 > { fn fmt (& self , f : & mut Formatter < '_ >) -> FmtResult { f . debug_struct ("Fanout") . field ("sink1" , & self . sink1) . field ("sink2" , & self . sink2) . finish () } }
    };
}

impl_927!();