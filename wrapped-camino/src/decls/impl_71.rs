macro_rules! deps {
    () => {
        ReadDirUtf8!();
        Utf8DirEntry!();
    };
}

macro_rules! impl_71 {
    () => {
        deps!();
        impl Iterator for ReadDirUtf8 { type Item = io :: Result < Utf8DirEntry > ; fn next (& mut self) -> Option < io :: Result < Utf8DirEntry > > { self . inner . next () . map (| entry | entry . and_then (Utf8DirEntry :: new)) } }
    };
}

impl_71!();