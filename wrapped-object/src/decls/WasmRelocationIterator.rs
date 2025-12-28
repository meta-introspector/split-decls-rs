macro_rules! deps {
    () => {
        WasmSection!();
    };
}

macro_rules! WasmRelocationIterator {
    () => {
        deps!();
        # [doc = " An iterator for the relocations for a [`WasmSection`]."] # [doc = ""] # [doc = " This is a stub that doesn't implement any functionality."] # [derive (Debug)] pub struct WasmRelocationIterator < 'data , 'file , R = & 'data [u8] > (PhantomData < (& 'data () , & 'file () , R) > ,) ;
    };
}

WasmRelocationIterator!()