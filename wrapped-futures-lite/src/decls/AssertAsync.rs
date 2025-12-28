macro_rules! deps {
    () => {
        AsyncReadExt!();
    };
}

macro_rules! AssertAsync {
    () => {
        deps!();
        # [doc = " Asserts that a type implementing [`std::io`] traits can be used as an async type."] # [doc = ""] # [doc = " The underlying I/O handle should never block nor return the [`ErrorKind::WouldBlock`] error."] # [doc = " This is usually the case for in-memory buffered I/O."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use futures_lite::io::{AssertAsync, AsyncReadExt};"] # [doc = ""] # [doc = " let reader: &[u8] = b\"hello\";"] # [doc = ""] # [doc = " # spin_on::spin_on(async {"] # [doc = " let mut async_reader = AssertAsync::new(reader);"] # [doc = " let mut contents = String::new();"] # [doc = ""] # [doc = " // This line works in async manner - note that there is await:"] # [doc = " async_reader.read_to_string(&mut contents).await?;"] # [doc = " # std::io::Result::Ok(()) });"] # [doc = " ```"] # [derive (Debug , Copy , Clone , Eq , PartialEq , Ord , PartialOrd , Hash)] pub struct AssertAsync < T > (T) ;
    };
}

AssertAsync!();