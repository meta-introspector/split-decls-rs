macro_rules! ImportType {
    () => {
        # [doc = " The kind of import symbol."] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub enum ImportType { # [doc = " An executable code symbol."] Code , # [doc = " A data symbol."] Data , # [doc = " A constant value."] Const , }
    };
}

ImportType!();