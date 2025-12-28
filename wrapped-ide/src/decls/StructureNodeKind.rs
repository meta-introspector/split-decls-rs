macro_rules! StructureNodeKind {
    () => {
        # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash , PartialOrd , Ord)] pub enum StructureNodeKind { SymbolKind (SymbolKind) , ExternBlock , Region , }
    };
}

StructureNodeKind!()