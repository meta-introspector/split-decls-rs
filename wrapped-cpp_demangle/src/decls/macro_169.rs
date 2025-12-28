macro_rules! macro_169 {
    () => {
        define_vocabulary ! { # [doc = " A <ref-qualifier> production."] # [doc = ""] # [doc = " ```text"] # [doc = " <ref-qualifier> ::= R   # & ref-qualifier"] # [doc = "                 ::= O   # && ref-qualifier"] # [doc = " ```"] # [derive (Clone , Debug , PartialEq , Eq)] pub enum RefQualifier { LValueRef (b"R" , "&") , RValueRef (b"O" , "&&") } }
    };
}

macro_169!()