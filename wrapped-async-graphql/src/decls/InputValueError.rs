macro_rules! deps {
    () => {
        ErrorExtensionValues!();
    };
}

macro_rules! InputValueError {
    () => {
        deps!();
        # [doc = " An error parsing an input value."] # [doc = ""] # [doc = " This type is generic over T as it uses T's type name when converting to a"] # [doc = " regular error."] # [derive (Debug)] pub struct InputValueError < T > { message : String , extensions : Option < ErrorExtensionValues > , phantom : PhantomData < T > , }
    };
}

InputValueError!();