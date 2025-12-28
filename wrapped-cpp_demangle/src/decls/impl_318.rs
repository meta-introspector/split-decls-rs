macro_rules! deps {
    () => {
        Result!();
        Error!();
    };
}

macro_rules! impl_318 {
    () => {
        deps!();
        impl fmt :: Display for Error { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match * self { Error :: UnexpectedEnd => write ! (f , "mangled symbol ends abruptly") , Error :: UnexpectedText => write ! (f , "mangled symbol is not well-formed") , Error :: BadBackReference => write ! (f , "back reference that is out-of-bounds of the substitution table") , Error :: BadTemplateArgReference => write ! (f , "reference to a template arg that is either out-of-bounds, or in a context \
                 without template args") , Error :: ForwardTemplateArgReference => write ! (f , "reference to a template arg from itself or a later template arg") , Error :: BadFunctionArgReference => write ! (f , "reference to a function arg that is either out-of-bounds, or in a context \
                 without function args") , Error :: BadLeafNameReference => write ! (f , "reference to a leaf name in a context where there is no current leaf name") , Error :: Overflow => write ! (f , "an overflow or underflow would occur when parsing an integer in a mangled \
                 symbol") , Error :: TooMuchRecursion => { write ! (f , "encountered too much recursion when demangling symbol") } } } }
    };
}

impl_318!()