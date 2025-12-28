macro_rules! deps {
    () => {
        BStr!();
    };
}

macro_rules! join {
    () => {
        deps!();
        # [doc = " Join the elements given by the iterator with the given separator into a"] # [doc = " single `Vec<u8>`."] # [doc = ""] # [doc = " Both the separator and the elements may be any type that can be cheaply"] # [doc = " converted into an `&[u8]`. This includes, but is not limited to,"] # [doc = " `&str`, `&BStr` and `&[u8]` itself."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Basic usage:"] # [doc = ""] # [doc = " ```"] # [doc = " use bstr;"] # [doc = ""] # [doc = " let s = bstr::join(\",\", &[\"foo\", \"bar\", \"baz\"]);"] # [doc = " assert_eq!(s, \"foo,bar,baz\".as_bytes());"] # [doc = " ```"] # [inline] pub fn join < B , T , I > (separator : B , elements : I) -> Vec < u8 > where B : AsRef < [u8] > , T : AsRef < [u8] > , I : IntoIterator < Item = T > , { let mut it = elements . into_iter () ; let mut dest = vec ! [] ; match it . next () { None => return dest , Some (first) => { dest . push_str (first) ; } } for element in it { dest . push_str (& separator) ; dest . push_str (element) ; } dest }
    };
}

join!()