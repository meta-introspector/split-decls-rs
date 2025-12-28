macro_rules! deps {
    () => {
        LineAnnotation!();
    };
}

macro_rules! AnnotatedLineInfo {
    () => {
        deps!();
        # [derive (Debug)] pub (crate) struct AnnotatedLineInfo < 'a > { pub (crate) line : & 'a str , pub (crate) line_index : usize , pub (crate) annotations : Vec < LineAnnotation < 'a > > , pub (crate) keep : bool , }
    };
}

AnnotatedLineInfo!()