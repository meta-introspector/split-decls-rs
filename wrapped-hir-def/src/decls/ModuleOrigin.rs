macro_rules! deps {
    () => {
        TreeId!();
    };
}

macro_rules! ModuleOrigin {
    () => {
        deps!();
        # [derive (Debug , PartialEq , Eq , Clone , Copy , Hash)] pub enum ModuleOrigin { CrateRoot { definition : EditionedFileId , } , # [doc = " Note that non-inline modules, by definition, live inside non-macro file."] File { is_mod_rs : bool , declaration : FileAstId < ast :: Module > , declaration_tree_id : TreeId , definition : EditionedFileId , } , Inline { definition_tree_id : TreeId , definition : FileAstId < ast :: Module > , } , # [doc = " Pseudo-module introduced by a block scope (contains only inner items)."] BlockExpr { id : BlockId , block : AstId < ast :: BlockExpr > , } , }
    };
}

ModuleOrigin!();