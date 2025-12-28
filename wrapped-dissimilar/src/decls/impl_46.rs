macro_rules! deps {
    () => {
        Chunk!();
    };
}

macro_rules! impl_46 {
    () => {
        deps!();
        impl Debug for Chunk < '_ > { fn fmt (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { let (name , text) = match * self { Chunk :: Equal (text) => ("Equal" , text) , Chunk :: Delete (text) => ("Delete" , text) , Chunk :: Insert (text) => ("Insert" , text) , } ; write ! (formatter , "{}({:?})" , name , text) } }
    };
}

impl_46!();