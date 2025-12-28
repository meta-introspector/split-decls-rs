macro_rules! SyntaxFixupUndoInfo {
    () => {
        # [doc = " This is the information needed to reverse the fixups."] # [derive (Clone , Debug , Default , PartialEq , Eq)] pub struct SyntaxFixupUndoInfo { original : Option < Arc < Box < [TopSubtree] > > > , }
    };
}

SyntaxFixupUndoInfo!()