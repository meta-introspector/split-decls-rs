macro_rules! Nonce {
    () => {
        # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub struct Nonce (usize) ;
    };
}

Nonce!()