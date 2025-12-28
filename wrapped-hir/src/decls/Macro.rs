macro_rules! Macro {
    () => {
        # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub struct Macro { pub (crate) id : MacroId , }
    };
}

Macro!();