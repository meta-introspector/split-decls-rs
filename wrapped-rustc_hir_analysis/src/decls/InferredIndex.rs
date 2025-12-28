macro_rules! InferredIndex {
    () => {
        # [derive (Copy , Clone , Debug)] pub (crate) struct InferredIndex (pub usize) ;
    };
}

InferredIndex!()