macro_rules! deps {
    () => {
        FileRange!();
        ExpandDatabase!();
        InFile!();
    };
}

macro_rules! impl_98 {
    () => {
        deps!();
        impl InFile < & SyntaxNode > { # [doc = " Attempts to map the syntax node back up its macro calls."] pub fn original_file_range_opt (self , db : & dyn db :: ExpandDatabase ,) -> Option < (FileRange , SyntaxContext) > { self . borrow () . map (SyntaxNode :: text_range) . original_node_file_range_opt (db) } }
    };
}

impl_98!();