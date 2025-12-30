// Generated macro for impl_905 (impl)
macro_rules! Depcrate_packetimpl_905 {
() => {
// Module: crate::packet
// Provides: {"impl_905"}
// Dependencies: {}
impl std :: fmt :: Debug for Header < '_ > { fn fmt (& self , f : & mut std :: fmt :: Formatter) -> std :: fmt :: Result { write ! (f , "{:?}" , self . ty) ? ; if self . ty != Type :: Short { write ! (f , " version={:x}" , self . version) ? ; } write ! (f , " dcid={:?}" , self . dcid) ? ; if self . ty != Type :: Short { write ! (f , " scid={:?}" , self . scid) ? ; } if let Some (ref token) = self . token { write ! (f , " token=") ? ; for b in token { write ! (f , "{b:02x}") ? ; } } if let Some (ref versions) = self . versions { write ! (f , " versions={versions:x?}") ? ; } if self . ty == Type :: Short { write ! (f , " key_phase={}" , self . key_phase) ? ; } Ok (()) } }
};
}
