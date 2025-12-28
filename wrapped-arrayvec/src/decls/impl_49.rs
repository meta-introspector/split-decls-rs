macro_rules! deps {
    () => {
        ArrayVec!();
    };
}

macro_rules! impl_49 {
    () => {
        deps!();
        # [cfg (feature = "zeroize")] # [doc = " \"Best efforts\" zeroing of the `ArrayVec`'s buffer when the `zeroize` feature is enabled."] # [doc = ""] # [doc = " The length is set to 0, and the buffer is dropped and zeroized."] # [doc = " Cannot ensure that previous moves of the `ArrayVec` did not leave values on the stack."] # [doc = ""] # [doc = " ```"] # [doc = " use arrayvec::ArrayVec;"] # [doc = " use zeroize::Zeroize;"] # [doc = " let mut array = ArrayVec::from([1, 2, 3]);"] # [doc = " array.zeroize();"] # [doc = " assert_eq!(array.len(), 0);"] # [doc = " let data = unsafe { core::slice::from_raw_parts(array.as_ptr(), array.capacity()) };"] # [doc = " assert_eq!(data, [0, 0, 0]);"] # [doc = " ```"] impl < Z : zeroize :: Zeroize , const CAP : usize > zeroize :: Zeroize for ArrayVec < Z , CAP > { fn zeroize (& mut self) { self . iter_mut () . zeroize () ; self . clear () ; self . xs . zeroize () ; } }
    };
}

impl_49!();