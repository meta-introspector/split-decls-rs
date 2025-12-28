macro_rules! deps {
    () => {
        BuiltinUint!();
        BuiltinInt!();
        Literal!();
        FloatTypeWrapper!();
        BuiltinFloat!();
    };
}

macro_rules! impl_225 {
    () => {
        deps!();
        impl From < ast :: LiteralKind > for Literal { fn from (ast_lit_kind : ast :: LiteralKind) -> Self { use ast :: LiteralKind ; match ast_lit_kind { LiteralKind :: IntNumber (lit) => { if let builtin @ Some (_) = lit . suffix () . and_then (BuiltinFloat :: from_suffix) { Literal :: Float (FloatTypeWrapper :: new (Symbol :: intern (& lit . value_string ())) , builtin ,) } else if let builtin @ Some (_) = lit . suffix () . and_then (BuiltinUint :: from_suffix) { Literal :: Uint (lit . value () . unwrap_or (0) , builtin) } else { let builtin = lit . suffix () . and_then (BuiltinInt :: from_suffix) ; Literal :: Int (lit . value () . unwrap_or (0) as i128 , builtin) } } LiteralKind :: FloatNumber (lit) => { let ty = lit . suffix () . and_then (BuiltinFloat :: from_suffix) ; Literal :: Float (FloatTypeWrapper :: new (Symbol :: intern (& lit . value_string ())) , ty) } LiteralKind :: ByteString (bs) => { let text = bs . value () . map_or_else (| _ | Default :: default () , Box :: from) ; Literal :: ByteString (text) } LiteralKind :: String (s) => { let text = s . value () . map_or_else (| _ | Symbol :: empty () , | it | Symbol :: intern (& it)) ; Literal :: String (text) } LiteralKind :: CString (s) => { let text = s . value () . map_or_else (| _ | Default :: default () , Box :: from) ; Literal :: CString (text) } LiteralKind :: Byte (b) => { Literal :: Uint (b . value () . unwrap_or_default () as u128 , Some (BuiltinUint :: U8)) } LiteralKind :: Char (c) => Literal :: Char (c . value () . unwrap_or_default ()) , LiteralKind :: Bool (val) => Literal :: Bool (val) , } } }
    };
}

impl_225!();