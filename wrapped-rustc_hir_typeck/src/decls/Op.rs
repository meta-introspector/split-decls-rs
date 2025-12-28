macro_rules! Op {
    () => {
        # [doc = " An assignment op (e.g. `a += b`), or a binary op (e.g. `a + b`)."] # [derive (Clone , Copy , Debug , PartialEq)] enum Op { BinOp (hir :: BinOp) , AssignOp (hir :: AssignOp) , }
    };
}

Op!();