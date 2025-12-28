macro_rules! ModuleSource {
    () => {
        # [derive (Debug , Clone , PartialEq , Eq)] pub enum ModuleSource { SourceFile (ast :: SourceFile) , Module (ast :: Module) , BlockExpr (ast :: BlockExpr) , }
    };
}

ModuleSource!()