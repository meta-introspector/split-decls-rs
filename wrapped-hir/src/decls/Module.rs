macro_rules! Module {
    () => {
        # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub struct Module { pub (crate) id : ModuleId , }
    };
}

Module!();