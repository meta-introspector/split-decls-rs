macro_rules! deps {
    () => {
        XcoffComdatIterator!();
        Item!();
        FileHeader!();
        ReadRef!();
        XcoffComdat!();
    };
}

macro_rules! impl_848 {
    () => {
        deps!();
        impl < 'data , 'file , Xcoff , R > Iterator for XcoffComdatIterator < 'data , 'file , Xcoff , R > where Xcoff : FileHeader , R : ReadRef < 'data > , { type Item = XcoffComdat < 'data , 'file , Xcoff , R > ; # [inline] fn next (& mut self) -> Option < Self :: Item > { None } }
    };
}

impl_848!();