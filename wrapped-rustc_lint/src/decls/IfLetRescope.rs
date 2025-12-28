macro_rules! IfLetRescope {
    () => {
        # [doc = " Lint for potential change in program semantics of `if let`s"] # [derive (Default)] pub (crate) struct IfLetRescope { skip : HirIdSet , }
    };
}

IfLetRescope!();