macro_rules! deps {
    () => {
        TextEdit!();
        SourceChange!();
    };
}

macro_rules! impl_183 {
    () => {
        deps!();
        impl From < IntMap < FileId , TextEdit > > for SourceChange { fn from (source_file_edits : IntMap < FileId , TextEdit >) -> SourceChange { let source_file_edits = source_file_edits . into_iter () . map (| (file_id , edit) | (file_id , (edit , None))) . collect () ; SourceChange { source_file_edits , file_system_edits : Vec :: new () , is_snippet : false , .. SourceChange :: default () } } }
    };
}

impl_183!();