macro_rules! TestAttr {
    () => {
        # [derive (Debug , Copy , Clone , Hash , PartialEq , Eq)] pub struct TestAttr { pub ignore : bool , }
    };
}

TestAttr!();