macro_rules! deps {
    () => {
        OperationsIterInner!();
    };
}

macro_rules! OperationsIter {
    () => {
        deps!();
        # [doc = " An iterator over the operations of a document."] # [derive (Debug , Clone)] pub struct OperationsIter < 'a > (OperationsIterInner < 'a >) ;
    };
}

OperationsIter!();