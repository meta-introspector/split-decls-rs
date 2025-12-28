macro_rules! deps {
    () => {
        LitIntType!();
        LitKind!();
        LitFloatType!();
    };
}

macro_rules! impl_120 {
    () => {
        deps!();
        impl LitKind { pub fn str (& self) -> Option < Symbol > { match * self { LitKind :: Str (s , _) => Some (s) , _ => None , } } # [doc = " Returns `true` if this literal is a string."] pub fn is_str (& self) -> bool { matches ! (self , LitKind :: Str (..)) } # [doc = " Returns `true` if this literal is byte literal string."] pub fn is_bytestr (& self) -> bool { matches ! (self , LitKind :: ByteStr (..)) } # [doc = " Returns `true` if this is a numeric literal."] pub fn is_numeric (& self) -> bool { matches ! (self , LitKind :: Int (..) | LitKind :: Float (..)) } # [doc = " Returns `true` if this literal has no suffix."] # [doc = " Note: this will return true for literals with prefixes such as raw strings and byte strings."] pub fn is_unsuffixed (& self) -> bool { ! self . is_suffixed () } # [doc = " Returns `true` if this literal has a suffix."] pub fn is_suffixed (& self) -> bool { match * self { LitKind :: Int (_ , LitIntType :: Signed (..) | LitIntType :: Unsigned (..)) | LitKind :: Float (_ , LitFloatType :: Suffixed (..)) => true , LitKind :: Str (..) | LitKind :: ByteStr (..) | LitKind :: CStr (..) | LitKind :: Byte (..) | LitKind :: Char (..) | LitKind :: Int (_ , LitIntType :: Unsuffixed) | LitKind :: Float (_ , LitFloatType :: Unsuffixed) | LitKind :: Bool (..) | LitKind :: Err (_) => false , } } }
    };
}

impl_120!()