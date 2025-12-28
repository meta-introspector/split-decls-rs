macro_rules! deps {
    () => {
        SvalAttribute!();
    };
}

macro_rules! ensure_missing {
    () => {
        deps!();
        pub (crate) fn ensure_missing < T : SvalAttribute > (ctxt : & str , request : T , attrs : & [Attribute]) { let key = request . key () . to_owned () ; if get_unchecked :: < T > (ctxt , request , attrs) . is_some () { panic ! ("unsupported attribute `{}` on {}" , key , ctxt) ; } }
    };
}

ensure_missing!();