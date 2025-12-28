macro_rules! deps {
    () => {
        Chain!();
        Buf!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl < T , U > Chain < T , U > { # [doc = " Creates a new `Chain` sequencing the provided values."] pub (crate) fn new (a : T , b : U) -> Chain < T , U > { Chain { a , b } } # [doc = " Gets a reference to the first underlying `Buf`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use bytes::Buf;"] # [doc = ""] # [doc = " let buf = (&b\"hello\"[..])"] # [doc = "     .chain(&b\"world\"[..]);"] # [doc = ""] # [doc = " assert_eq!(buf.first_ref()[..], b\"hello\"[..]);"] # [doc = " ```"] pub fn first_ref (& self) -> & T { & self . a } # [doc = " Gets a mutable reference to the first underlying `Buf`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use bytes::Buf;"] # [doc = ""] # [doc = " let mut buf = (&b\"hello\"[..])"] # [doc = "     .chain(&b\"world\"[..]);"] # [doc = ""] # [doc = " buf.first_mut().advance(1);"] # [doc = ""] # [doc = " let full = buf.copy_to_bytes(9);"] # [doc = " assert_eq!(full, b\"elloworld\"[..]);"] # [doc = " ```"] pub fn first_mut (& mut self) -> & mut T { & mut self . a } # [doc = " Gets a reference to the last underlying `Buf`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use bytes::Buf;"] # [doc = ""] # [doc = " let buf = (&b\"hello\"[..])"] # [doc = "     .chain(&b\"world\"[..]);"] # [doc = ""] # [doc = " assert_eq!(buf.last_ref()[..], b\"world\"[..]);"] # [doc = " ```"] pub fn last_ref (& self) -> & U { & self . b } # [doc = " Gets a mutable reference to the last underlying `Buf`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use bytes::Buf;"] # [doc = ""] # [doc = " let mut buf = (&b\"hello \"[..])"] # [doc = "     .chain(&b\"world\"[..]);"] # [doc = ""] # [doc = " buf.last_mut().advance(1);"] # [doc = ""] # [doc = " let full = buf.copy_to_bytes(10);"] # [doc = " assert_eq!(full, b\"hello orld\"[..]);"] # [doc = " ```"] pub fn last_mut (& mut self) -> & mut U { & mut self . b } # [doc = " Consumes this `Chain`, returning the underlying values."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use bytes::Buf;"] # [doc = ""] # [doc = " let chain = (&b\"hello\"[..])"] # [doc = "     .chain(&b\"world\"[..]);"] # [doc = ""] # [doc = " let (first, last) = chain.into_inner();"] # [doc = " assert_eq!(first[..], b\"hello\"[..]);"] # [doc = " assert_eq!(last[..], b\"world\"[..]);"] # [doc = " ```"] pub fn into_inner (self) -> (T , U) { (self . a , self . b) } }
    };
}

impl_23!()