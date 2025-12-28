macro_rules! deps {
    () => {
        HeckeOperator!();
    };
}

macro_rules! DummyHeckeOperator {
    () => {
        deps!();
        # [doc = " A dummy implementation of `HeckeOperator` for testing."] # [derive (Debug , Default)] pub struct DummyHeckeOperator ;
    };
}

DummyHeckeOperator!()