// Generated macro for kind_of (function)
macro_rules! Depcrate_implskind_of {
() => {
// Module: crate::impls
// Provides: {"kind_of"}
// Dependencies: {}
fn kind_of (lit : & Literal < String >) -> TokenKind { match lit { Literal :: String (_) => TokenKind :: StringLit , Literal :: Bool (_) => TokenKind :: BoolLit , Literal :: Integer (_) => TokenKind :: IntegerLit , Literal :: Float (_) => TokenKind :: FloatLit , Literal :: Char (_) => TokenKind :: CharLit , Literal :: Byte (_) => TokenKind :: ByteLit , Literal :: ByteString (_) => TokenKind :: ByteStringLit , Literal :: CString (_) => TokenKind :: CStringLit , } }
};
}
