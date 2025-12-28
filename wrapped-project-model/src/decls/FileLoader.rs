macro_rules! FileLoader {
    () => {
        pub type FileLoader < 'a > = & 'a mut dyn for < 'b > FnMut (& 'b AbsPath) -> Option < FileId > ;
    };
}

FileLoader!();