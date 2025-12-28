macro_rules! deps {
    () => {
        SourceChange!();
        FileSystemEdit!();
    };
}

macro_rules! impl_182 {
    () => {
        deps!();
        impl Extend < FileSystemEdit > for SourceChange { fn extend < T : IntoIterator < Item = FileSystemEdit > > (& mut self , iter : T) { iter . into_iter () . for_each (| edit | self . push_file_system_edit (edit)) ; } }
    };
}

impl_182!();