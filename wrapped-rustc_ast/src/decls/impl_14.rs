macro_rules! deps {
    () => {
        PathSegment!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl PartialEq < Symbol > for PathSegment { # [inline] fn eq (& self , name : & Symbol) -> bool { self . args . is_none () && self . ident . name == * name } }
    };
}

impl_14!();