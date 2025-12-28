macro_rules! deps {
    () => {
        TextEdit!();
        SourceChange!();
    };
}

macro_rules! impl_180 {
    () => {
        deps!();
        impl Extend < (FileId , TextEdit) > for SourceChange { fn extend < T : IntoIterator < Item = (FileId , TextEdit) > > (& mut self , iter : T) { self . extend (iter . into_iter () . map (| (file_id , edit) | (file_id , (edit , None)))) } }
    };
}

impl_180!();