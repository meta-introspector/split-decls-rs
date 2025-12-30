// Generated macro for impl_222 (impl)
macro_rules! Depcrate_clipboardimpl_222 {
() => {
// Module: crate::clipboard
// Provides: {"impl_222"}
// Dependencies: {}
impl < T : AsRef < [u8] > > Command for CopyToClipboard < T > { fn write_ansi (& self , f : & mut impl fmt :: Write) -> fmt :: Result { write ! (f , osc ! ("52;{destination};{encoded_text}") , destination = self . destination . to_osc52_pc () , encoded_text = BASE64_STANDARD . encode (& self . content)) } # [cfg (windows)] fn execute_winapi (& self) -> std :: io :: Result < () > { use std :: io ; Err (io :: Error :: new (io :: ErrorKind :: Unsupported , "Copying is not implemented for the Windows API." ,)) } }
};
}
