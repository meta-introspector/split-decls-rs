macro_rules! Arch {
    () => {
        # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub enum Arch { Wasm32 , Wasm64 , Other , }
    };
}

Arch!()