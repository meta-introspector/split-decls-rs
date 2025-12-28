macro_rules! deps {
    () => {
        TextEdit!();
        SnippetEdit!();
        ChangeAnnotation!();
        FileSystemEdit!();
        ChangeAnnotationId!();
        SourceChange!();
    };
}

macro_rules! impl_179 {
    () => {
        deps!();
        impl SourceChange { pub fn from_text_edit (file_id : impl Into < FileId > , edit : TextEdit) -> Self { SourceChange { source_file_edits : iter :: once ((file_id . into () , (edit , None))) . collect () , .. Default :: default () } } pub fn insert_annotation (& mut self , annotation : ChangeAnnotation) -> ChangeAnnotationId { let id = ChangeAnnotationId (self . next_annotation_id) ; self . next_annotation_id += 1 ; self . annotations . insert (id , annotation) ; id } # [doc = " Inserts a [`TextEdit`] for the given [`FileId`]. This properly handles merging existing"] # [doc = " edits for a file if some already exist."] pub fn insert_source_edit (& mut self , file_id : impl Into < FileId > , edit : TextEdit) { self . insert_source_and_snippet_edit (file_id . into () , edit , None) } # [doc = " Inserts a [`TextEdit`] and potentially a [`SnippetEdit`] for the given [`FileId`]."] # [doc = " This properly handles merging existing edits for a file if some already exist."] pub fn insert_source_and_snippet_edit (& mut self , file_id : impl Into < FileId > , edit : TextEdit , snippet_edit : Option < SnippetEdit > ,) { match self . source_file_edits . entry (file_id . into ()) { Entry :: Occupied (mut entry) => { let value = entry . get_mut () ; never ! (value . 0 . union (edit) . is_err () , "overlapping edits for same file") ; never ! (value . 1 . is_some () && snippet_edit . is_some () , "overlapping snippet edits for same file") ; if value . 1 . is_none () { value . 1 = snippet_edit ; } } Entry :: Vacant (entry) => { entry . insert ((edit , snippet_edit)) ; } } } pub fn push_file_system_edit (& mut self , edit : FileSystemEdit) { self . file_system_edits . push (edit) ; } pub fn get_source_and_snippet_edit (& self , file_id : FileId ,) -> Option < & (TextEdit , Option < SnippetEdit >) > { self . source_file_edits . get (& file_id) } pub fn merge (mut self , other : SourceChange) -> SourceChange { self . extend (other . source_file_edits) ; self . extend (other . file_system_edits) ; self . is_snippet |= other . is_snippet ; self } }
    };
}

impl_179!()