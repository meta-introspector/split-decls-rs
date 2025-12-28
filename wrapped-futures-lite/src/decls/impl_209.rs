macro_rules! deps {
    () => {
        BlockOn!();
    };
}

macro_rules! impl_209 {
    () => {
        deps!();
        impl < T > BlockOn < T > { # [doc = " Wraps an async I/O handle into a blocking interface."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use futures_lite::io::BlockOn;"] # [doc = " use futures_lite::pin;"] # [doc = ""] # [doc = " let reader: &[u8] = b\"hello\";"] # [doc = " pin!(reader);"] # [doc = ""] # [doc = " let blocking_reader = BlockOn::new(reader);"] # [doc = " ```"] pub fn new (io : T) -> BlockOn < T > { BlockOn (io) } # [doc = " Gets a reference to the async I/O handle."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use futures_lite::io::BlockOn;"] # [doc = " use futures_lite::pin;"] # [doc = ""] # [doc = " let reader: &[u8] = b\"hello\";"] # [doc = " pin!(reader);"] # [doc = ""] # [doc = " let blocking_reader = BlockOn::new(reader);"] # [doc = " let r = blocking_reader.get_ref();"] # [doc = " ```"] pub fn get_ref (& self) -> & T { & self . 0 } # [doc = " Gets a mutable reference to the async I/O handle."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use futures_lite::io::BlockOn;"] # [doc = " use futures_lite::pin;"] # [doc = ""] # [doc = " let reader: &[u8] = b\"hello\";"] # [doc = " pin!(reader);"] # [doc = ""] # [doc = " let mut blocking_reader = BlockOn::new(reader);"] # [doc = " let r = blocking_reader.get_mut();"] # [doc = " ```"] pub fn get_mut (& mut self) -> & mut T { & mut self . 0 } # [doc = " Extracts the inner async I/O handle."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use futures_lite::io::BlockOn;"] # [doc = " use futures_lite::pin;"] # [doc = ""] # [doc = " let reader: &[u8] = b\"hello\";"] # [doc = " pin!(reader);"] # [doc = ""] # [doc = " let blocking_reader = BlockOn::new(reader);"] # [doc = " let inner = blocking_reader.into_inner();"] # [doc = " ```"] pub fn into_inner (self) -> T { self . 0 } }
    };
}

impl_209!()