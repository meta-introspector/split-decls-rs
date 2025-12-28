macro_rules! deps {
    () => {
        PeSection!();
    };
}

macro_rules! PeRelocationIterator {
    () => {
        deps!();
        # [doc = " An iterator for the relocations in an [`PeSection`]."] # [doc = ""] # [doc = " This is a stub that doesn't implement any functionality."] # [derive (Debug)] pub struct PeRelocationIterator < 'data , 'file , R = & 'data [u8] > (PhantomData < (& 'data () , & 'file () , R) > ,) ;
    };
}

PeRelocationIterator!();