macro_rules! deps {
    () => {
        Cursor!();
    };
}

macro_rules! impl_1217 {
    () => {
        deps!();
        impl < R : AsyncRead > Take < R > { pub (super) fn new (inner : R , limit : u64) -> Self { Self { inner , limit } } # [doc = " Returns the remaining number of bytes that can be"] # [doc = " read before this instance will return EOF."] # [doc = ""] # [doc = " # Note"] # [doc = ""] # [doc = " This instance may reach `EOF` after reading fewer bytes than indicated by"] # [doc = " this method if the underlying [`AsyncRead`] instance reaches EOF."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # futures::executor::block_on(async {"] # [doc = " use futures::io::{AsyncReadExt, Cursor};"] # [doc = ""] # [doc = " let reader = Cursor::new(&b\"12345678\"[..]);"] # [doc = " let mut buffer = [0; 2];"] # [doc = ""] # [doc = " let mut take = reader.take(4);"] # [doc = " let n = take.read(&mut buffer).await?;"] # [doc = ""] # [doc = " assert_eq!(take.limit(), 2);"] # [doc = " # Ok::<(), Box<dyn std::error::Error>>(()) }).unwrap();"] # [doc = " ```"] pub fn limit (& self) -> u64 { self . limit } # [doc = " Sets the number of bytes that can be read before this instance will"] # [doc = " return EOF. This is the same as constructing a new `Take` instance, so"] # [doc = " the amount of bytes read and the previous limit value don't matter when"] # [doc = " calling this method."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " # futures::executor::block_on(async {"] # [doc = " use futures::io::{AsyncReadExt, Cursor};"] # [doc = ""] # [doc = " let reader = Cursor::new(&b\"12345678\"[..]);"] # [doc = " let mut buffer = [0; 4];"] # [doc = ""] # [doc = " let mut take = reader.take(4);"] # [doc = " let n = take.read(&mut buffer).await?;"] # [doc = ""] # [doc = " assert_eq!(n, 4);"] # [doc = " assert_eq!(take.limit(), 0);"] # [doc = ""] # [doc = " take.set_limit(10);"] # [doc = " let n = take.read(&mut buffer).await?;"] # [doc = " assert_eq!(n, 4);"] # [doc = ""] # [doc = " # Ok::<(), Box<dyn std::error::Error>>(()) }).unwrap();"] # [doc = " ```"] pub fn set_limit (& mut self , limit : u64) { self . limit = limit } delegate_access_inner ! (inner , R , ()) ; }
    };
}

impl_1217!()