macro_rules! deps {
    () => {
        TextEdit!();
        SourceChange!();
    };
}

macro_rules! impl_184 {
    () => {
        deps!();
        impl FromIterator < (FileId , TextEdit) > for SourceChange { fn from_iter < T : IntoIterator < Item = (FileId , TextEdit) > > (iter : T) -> Self { let mut this = SourceChange :: default () ; this . extend (iter) ; this } }
    };
}

impl_184!();