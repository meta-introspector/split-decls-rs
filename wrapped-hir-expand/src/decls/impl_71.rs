macro_rules! deps {
    () => {
        HirFileRange!();
        FileRange!();
    };
}

macro_rules! impl_71 {
    () => {
        deps!();
        impl HirFileRange { pub fn file_range (self) -> Option < FileRange > { Some (FileRange { file_id : self . file_id . file_id () ? , range : self . range }) } }
    };
}

impl_71!()