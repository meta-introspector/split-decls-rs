macro_rules! ByteString {
    () => {
        # [doc = " A byte slice that is a string of an unknown encoding."] # [doc = ""] # [doc = " Uses copy-on-write to avoid unnecessary allocations. The bytes can be"] # [doc = " accessed as a slice using the `Deref` trait, or as a mutable `Vec` using the"] # [doc = " `to_mut` method."] # [doc = ""] # [doc = " Provides a `Debug` implementation that interprets the bytes as UTF-8."] # [derive (Default , Clone , PartialEq , Eq , Hash)] pub struct ByteString < 'a > (Cow < 'a , [u8] >) ;
    };
}

ByteString!();