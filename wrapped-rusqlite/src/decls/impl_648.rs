macro_rules! deps {
    () => {
        VTabLog!();
        VTabConnection!();
        VTabKind!();
        Result!();
        CreateVTab!();
    };
}

macro_rules! impl_648 {
    () => {
        deps!();
        impl CreateVTab < '_ > for VTabLog { const KIND : VTabKind = VTabKind :: Default ; fn create (db : & mut VTabConnection , aux : Option < & Self :: Aux > , args : & [& [u8]] ,) -> Result < (String , Self) > { Self :: connect_create (db , aux , args , true) } fn destroy (& self) -> Result < () > { println ! ("VTabLog::destroy({})" , self . i_inst) ; Ok (()) } }
    };
}

impl_648!()