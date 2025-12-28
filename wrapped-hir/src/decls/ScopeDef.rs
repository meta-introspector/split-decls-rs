macro_rules! deps {
    () => {
        Impl!();
        Local!();
        Adt!();
        ModuleDef!();
        GenericParam!();
        Label!();
    };
}

macro_rules! ScopeDef {
    () => {
        deps!();
        # [doc = " For IDE only"] # [derive (Copy , Clone , Debug , PartialEq , Eq , Hash)] pub enum ScopeDef { ModuleDef (ModuleDef) , GenericParam (GenericParam) , ImplSelfType (Impl) , AdtSelfType (Adt) , Local (Local) , Label (Label) , Unknown , }
    };
}

ScopeDef!();