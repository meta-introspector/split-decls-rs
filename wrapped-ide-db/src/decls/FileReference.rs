macro_rules! deps {
    () => {
        FileReferenceNode!();
    };
}

macro_rules! FileReference {
    () => {
        deps!();
        # [derive (Debug , Clone)] pub struct FileReference { # [doc = " The range of the reference in the original file"] pub range : TextRange , # [doc = " The node of the reference in the (macro-)file"] pub name : FileReferenceNode , pub category : ReferenceCategory , }
    };
}

FileReference!()