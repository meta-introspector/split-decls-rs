macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_319 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl error :: Error for Error { fn description (& self) -> & str { match * self { Error :: UnexpectedEnd => "mangled symbol ends abruptly" , Error :: UnexpectedText => "mangled symbol is not well-formed" , Error :: BadBackReference => { "back reference that is out-of-bounds of the substitution table" } Error :: BadTemplateArgReference => { "reference to a template arg that is either out-of-bounds, or in a context \
                 without template args" } Error :: ForwardTemplateArgReference => { "reference to a template arg from itself or a later template arg" } Error :: BadFunctionArgReference => { "reference to a function arg that is either out-of-bounds, or in a context \
                 without function args" } Error :: BadLeafNameReference => { "reference to a leaf name in a context where there is no current leaf name" } Error :: Overflow => { "an overflow or underflow would occur when parsing an integer in a mangled symbol" } Error :: TooMuchRecursion => "encountered too much recursion when demangling symbol" , } } }
    };
}

impl_319!();