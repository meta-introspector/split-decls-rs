macro_rules! deps {
    () => {
        FileSystemEdit!();
        SourceChange!();
    };
}

macro_rules! impl_193 {
    () => {
        deps!();
        impl From < FileSystemEdit > for SourceChange { fn from (edit : FileSystemEdit) -> SourceChange { SourceChange { source_file_edits : Default :: default () , file_system_edits : vec ! [edit] , is_snippet : false , .. SourceChange :: default () } } }
    };
}

impl_193!()