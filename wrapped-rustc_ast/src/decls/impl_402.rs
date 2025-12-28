macro_rules! deps {
    () => {
        LitKind!();
    };
}

macro_rules! impl_402 {
    () => {
        deps!();
        impl LitKind { # [doc = " An English article for the literal token kind."] pub fn article (self) -> & 'static str { match self { Integer | Err (_) => "an" , _ => "a" , } } pub fn descr (self) -> & 'static str { match self { Bool => "boolean" , Byte => "byte" , Char => "char" , Integer => "integer" , Float => "float" , Str | StrRaw (..) => "string" , ByteStr | ByteStrRaw (..) => "byte string" , CStr | CStrRaw (..) => "C string" , Err (_) => "error" , } } pub (crate) fn may_have_suffix (self) -> bool { matches ! (self , Integer | Float | Err (_)) } }
    };
}

impl_402!()