macro_rules! deps {
    () => {
        TextEdit!();
        SnippetEdit!();
        SourceChange!();
    };
}

macro_rules! impl_181 {
    () => {
        deps!();
        impl Extend < (FileId , (TextEdit , Option < SnippetEdit >)) > for SourceChange { fn extend < T : IntoIterator < Item = (FileId , (TextEdit , Option < SnippetEdit >)) > > (& mut self , iter : T ,) { iter . into_iter () . for_each (| (file_id , (edit , snippet_edit)) | { self . insert_source_and_snippet_edit (file_id , edit , snippet_edit) }) ; } }
    };
}

impl_181!();