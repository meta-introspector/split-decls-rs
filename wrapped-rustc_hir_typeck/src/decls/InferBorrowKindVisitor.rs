macro_rules! deps {
    () => {
        FnCtxt!();
    };
}

macro_rules! InferBorrowKindVisitor {
    () => {
        deps!();
        struct InferBorrowKindVisitor < 'a , 'tcx > { fcx : & 'a FnCtxt < 'a , 'tcx > , }
    };
}

InferBorrowKindVisitor!();