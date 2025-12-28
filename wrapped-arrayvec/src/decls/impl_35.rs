macro_rules! deps {
    () => {
        ArrayString!();
    };
}

macro_rules! impl_35 {
    () => {
        deps!();
        # [cfg (feature = "zeroize")] # [doc = " \"Best efforts\" zeroing of the `ArrayString`'s buffer when the `zeroize` feature is enabled."] # [doc = ""] # [doc = " The length is set to 0, and the buffer is dropped and zeroized."] # [doc = " Cannot ensure that previous moves of the `ArrayString` did not leave values on the stack."] # [doc = ""] # [doc = " ```"] # [doc = " use arrayvec::ArrayString;"] # [doc = " use zeroize::Zeroize;"] # [doc = " let mut string = ArrayString::<6>::from(\"foobar\").unwrap();"] # [doc = " string.zeroize();"] # [doc = " assert_eq!(string.len(), 0);"] # [doc = " unsafe { string.set_len(string.capacity()) };"] # [doc = " assert_eq!(&*string, \"\\0\\0\\0\\0\\0\\0\");"] # [doc = " ```"] impl < const CAP : usize > zeroize :: Zeroize for ArrayString < CAP > { fn zeroize (& mut self) { self . clear () ; self . xs . zeroize () ; } }
    };
}

impl_35!();