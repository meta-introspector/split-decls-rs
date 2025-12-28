macro_rules! deps {
    () => {
        ExpandDatabase!();
        InMacroFile!();
        FileRange!();
    };
}

macro_rules! impl_101 {
    () => {
        deps!();
        impl InMacroFile < TextSize > { pub fn original_file_range (self , db : & dyn db :: ExpandDatabase) -> (FileRange , SyntaxContext) { span_for_offset (db , & db . expansion_span_map (self . file_id) , self . value) } }
    };
}

impl_101!();