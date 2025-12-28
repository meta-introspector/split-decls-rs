macro_rules! FilterOp {
    () => {
        # [derive (Debug , Clone)] pub (crate) struct FilterOp { # [cfg (feature = "regex")] inner : regex :: Regex , # [cfg (not (feature = "regex"))] inner : String , }
    };
}

FilterOp!()