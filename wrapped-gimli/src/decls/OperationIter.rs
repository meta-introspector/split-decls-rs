macro_rules! deps {
    () => {
        Encoding!();
        Reader!();
    };
}

macro_rules! OperationIter {
    () => {
        deps!();
        # [doc = " An iterator for the operations in an expression."] # [derive (Debug , Clone , Copy)] pub struct OperationIter < R : Reader > { input : R , encoding : Encoding , }
    };
}

OperationIter!();