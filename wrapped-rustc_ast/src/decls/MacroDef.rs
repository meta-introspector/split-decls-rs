macro_rules! deps {
    () => {
        DelimArgs!();
        Walkable!();
    };
}

macro_rules! MacroDef {
    () => {
        deps!();
        # [doc = " Represents a macro definition."] # [derive (Clone , Encodable , Decodable , Debug , HashStable_Generic , Walkable)] pub struct MacroDef { pub body : Box < DelimArgs > , # [doc = " `true` if macro was defined with `macro_rules`."] pub macro_rules : bool , }
    };
}

MacroDef!()