macro_rules! TokenId {
    () => {
        # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub struct TokenId (usize) ;
    };
}

TokenId!()