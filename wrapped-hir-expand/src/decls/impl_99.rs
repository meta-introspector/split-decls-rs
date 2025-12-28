macro_rules! deps {
    () => {
        ExpandDatabase!();
        InMacroFile!();
        InFile!();
    };
}

macro_rules! impl_99 {
    () => {
        deps!();
        impl InMacroFile < SyntaxToken > { pub fn upmap_once (self , db : & dyn db :: ExpandDatabase ,) -> InFile < smallvec :: SmallVec < TextRange , 1 > > { self . file_id . expansion_info (db) . map_range_up_once (db , self . value . text_range ()) } }
    };
}

impl_99!()