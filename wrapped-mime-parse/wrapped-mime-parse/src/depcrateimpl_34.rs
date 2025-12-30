// Generated macro for impl_34 (impl)
macro_rules! Depcrateimpl_34 {
() => {
// Module: crate
// Provides: {"impl_34"}
// Dependencies: {}
impl fmt :: Debug for Byte { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match self . 0 { b'\n' => f . write_str ("'\\n'") , b'\r' => f . write_str ("'\\r'") , b'\t' => f . write_str ("'\\t'") , b'\\' => f . write_str ("'\\'") , b'\0' => f . write_str ("'\\0'") , 0x20 ..= 0x7f => write ! (f , "'{}'" , self . 0 as char) , _ => write ! (f , "'\\x{:02x}'" , self . 0) , } } }
};
}
