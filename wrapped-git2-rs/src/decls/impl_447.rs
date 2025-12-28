macro_rules! deps {
    () => {
        MessageTrailers!();
        Binding!();
        MessageTrailersIterator!();
    };
}

macro_rules! impl_447 {
    () => {
        deps!();
        impl MessageTrailers { fn new () -> MessageTrailers { crate :: init () ; unsafe { Binding :: from_raw (& mut raw :: git_message_trailer_array { trailers : ptr :: null_mut () , count : 0 , _trailer_block : ptr :: null_mut () , } as * mut _) } } fn iter (& self) -> MessageTrailersIterator < '_ > { MessageTrailersIterator { trailers : self , range : Range { start : 0 , end : self . raw . count , } , } } fn len (& self) -> usize { self . raw . count } }
    };
}

impl_447!();