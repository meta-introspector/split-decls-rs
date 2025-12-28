macro_rules! deps {
    () => {
        Substructure!();
        BlockOrExpr!();
    };
}

macro_rules! CombineSubstructureFunc {
    () => {
        deps!();
        # [doc = " Combine the values of all the fields together. The last argument is"] # [doc = " all the fields of all the structures."] pub (crate) type CombineSubstructureFunc < 'a > = Box < dyn FnMut (& ExtCtxt < '_ > , Span , & Substructure < '_ >) -> BlockOrExpr + 'a > ;
    };
}

CombineSubstructureFunc!()