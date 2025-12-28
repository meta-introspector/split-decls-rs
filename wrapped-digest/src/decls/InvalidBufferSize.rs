macro_rules! InvalidBufferSize {
    () => {
        # [doc = " Buffer length is not equal to hash output size."] # [derive (Default , Debug , Copy , Clone , Eq , PartialEq)] pub struct InvalidBufferSize ;
    };
}

InvalidBufferSize!()