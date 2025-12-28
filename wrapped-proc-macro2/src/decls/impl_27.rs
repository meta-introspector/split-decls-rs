macro_rules! deps {
    () => {
        TokenStream!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        # [doc = " Prints the token stream as a string that is supposed to be losslessly"] # [doc = " convertible back into the same token stream (modulo spans), except for"] # [doc = " possibly `TokenTree::Group`s with `Delimiter::None` delimiters and negative"] # [doc = " numeric literals."] impl Display for TokenStream { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { Display :: fmt (& self . inner , f) } }
    };
}

impl_27!()