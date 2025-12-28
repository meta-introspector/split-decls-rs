macro_rules! deps {
    () => {
        ModuleId!();
    };
}

macro_rules! BlockLoc {
    () => {
        deps!();
        # [derive (Debug , Hash , PartialEq , Eq , Clone)] pub struct BlockLoc { pub ast_id : AstId < ast :: BlockExpr > , # [doc = " The containing module."] pub module : ModuleId , }
    };
}

BlockLoc!()