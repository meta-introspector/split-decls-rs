macro_rules! DeriveHelper {
    () => {
        # [derive (Clone , Copy , Debug , PartialEq , Eq , Hash)] pub struct DeriveHelper { pub (crate) derive : MacroId , pub (crate) idx : u32 , }
    };
}

DeriveHelper!()