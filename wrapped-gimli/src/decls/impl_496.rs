macro_rules! deps {
    () => {
        Result!();
        Reader!();
        UnitRef!();
        MacroString!();
    };
}

macro_rules! impl_496 {
    () => {
        deps!();
        impl < R : Reader > MacroString < R > { # [doc = " Get the string slice from the macro entry."] pub fn string (& self , unit : UnitRef < '_ , R >) -> Result < R > { match self { MacroString :: Direct (s) => Ok (s . clone ()) , MacroString :: StringPointer (offset) => unit . string (* offset) , MacroString :: IndirectStringPointer (index) => { let str_offset = unit . string_offset (* index) ? ; unit . string (str_offset) } MacroString :: Supplementary (offset) => unit . sup_string (* offset) , } } }
    };
}

impl_496!()