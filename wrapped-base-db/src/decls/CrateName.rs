macro_rules! CrateName {
    () => {
        # [derive (Debug , Clone , PartialEq , Eq , Hash)] pub struct CrateName (Symbol) ;
    };
}

CrateName!()