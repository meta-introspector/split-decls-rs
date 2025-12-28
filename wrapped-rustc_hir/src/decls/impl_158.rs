macro_rules! deps {
    () => {
        AttrPath!();
        Path!();
    };
}

macro_rules! impl_158 {
    () => {
        deps!();
        impl AttrPath { pub fn from_ast (path : & ast :: Path) -> Self { AttrPath { segments : path . segments . iter () . map (| i | i . ident) . collect :: < Vec < _ > > () . into_boxed_slice () , span : path . span , } } }
    };
}

impl_158!()