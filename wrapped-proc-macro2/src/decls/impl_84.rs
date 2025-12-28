macro_rules! deps {
    () => {
        TokenStream!();
        RcVecBuilder!();
        LexError!();
        TokenTree!();
    };
}

macro_rules! impl_84 {
    () => {
        deps!();
        impl TokenStream { pub (crate) fn new () -> Self { TokenStream { inner : RcVecBuilder :: new () . build () , } } pub (crate) fn from_str_checked (src : & str) -> Result < Self , LexError > { let mut cursor = get_cursor (src) ; const BYTE_ORDER_MARK : & str = "\u{feff}" ; if cursor . starts_with (BYTE_ORDER_MARK) { cursor = cursor . advance (BYTE_ORDER_MARK . len ()) ; } parse :: token_stream (cursor) } # [cfg (feature = "proc-macro")] pub (crate) fn from_str_unchecked (src : & str) -> Self { Self :: from_str_checked (src) . unwrap () } pub (crate) fn is_empty (& self) -> bool { self . inner . len () == 0 } fn take_inner (self) -> RcVecBuilder < TokenTree > { let nodrop = ManuallyDrop :: new (self) ; unsafe { ptr :: read (& nodrop . inner) } . make_owned () } }
    };
}

impl_84!()