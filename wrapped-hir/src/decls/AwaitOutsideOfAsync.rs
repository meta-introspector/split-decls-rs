macro_rules! AwaitOutsideOfAsync {
    () => {
        # [derive (Debug)] pub struct AwaitOutsideOfAsync { pub node : InFile < AstPtr < ast :: AwaitExpr > > , pub location : String , }
    };
}

AwaitOutsideOfAsync!()