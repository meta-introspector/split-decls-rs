macro_rules! Buffer {
    () => {
        # [doc = " A shared or owned string buffer. Implemented for `String` and `&str`. *Implementation detail*."] # [doc = ""] # [doc = " This is trait is implementation detail of this library, cannot be"] # [doc = " implemented in other crates and is not subject to semantic versioning."] # [doc = " `litrs` only guarantees that this trait is implemented for `String` and"] # [doc = " `for<'a> &'a str`."] pub trait Buffer : sealed :: Sealed + Deref < Target = str > { # [doc = " This is `String` for `String`, and `Cow<'a, str>` for `&'a str`."] type Cow : From < String > + AsRef < str > + Borrow < str > + Deref < Target = str > ; # [doc (hidden)] fn into_cow (self) -> Self :: Cow ; # [doc = " This is `Vec<u8>` for `String`, and `Cow<'a, [u8]>` for `&'a str`."] type ByteCow : From < Vec < u8 > > + AsRef < [u8] > + Borrow < [u8] > + Deref < Target = [u8] > ; # [doc (hidden)] fn into_byte_cow (self) -> Self :: ByteCow ; # [doc = " Cuts away some characters at the beginning and some at the end. Given"] # [doc = " range has to be in bounds."] # [doc (hidden)] fn cut (self , range : Range < usize >) -> Self ; }
    };
}

Buffer!();