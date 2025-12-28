macro_rules! deps {
    () => {
        Group!();
    };
}

macro_rules! impl_47 {
    () => {
        deps!();
        # [doc = " Prints the group as a string that should be losslessly convertible back"] # [doc = " into the same group (modulo spans), except for possibly `TokenTree::Group`s"] # [doc = " with `Delimiter::None` delimiters."] impl Display for Group { fn fmt (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { Display :: fmt (& self . inner , formatter) } }
    };
}

impl_47!()