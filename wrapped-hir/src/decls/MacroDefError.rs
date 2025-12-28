macro_rules! deps {
    () => {
        Macro!();
    };
}

macro_rules! MacroDefError {
    () => {
        deps!();
        # [derive (Debug , Clone , Eq , PartialEq)] pub struct MacroDefError { pub node : InFile < AstPtr < ast :: Macro > > , pub message : String , pub name : Option < TextRange > , }
    };
}

MacroDefError!()