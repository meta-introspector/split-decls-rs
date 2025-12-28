macro_rules! Var {
    () => {
        # [doc = " Represents a `$var` in an SSR query."] # [derive (Debug , Clone , PartialEq , Eq , Hash)] pub (crate) struct Var (pub (crate) String) ;
    };
}

Var!();