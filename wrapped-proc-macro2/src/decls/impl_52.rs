macro_rules! deps {
    () => {
        Punct!();
    };
}

macro_rules! impl_52 {
    () => {
        deps!();
        # [doc = " Prints the punctuation character as a string that should be losslessly"] # [doc = " convertible back into the same character."] impl Display for Punct { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { Display :: fmt (& self . ch , f) } }
    };
}

impl_52!()