// Generated macro for impl_37 (impl)
macro_rules! Depcrate_infoimpl_37 {
() => {
// Module: crate::info
// Provides: {"impl_37"}
// Dependencies: {}
impl Display for Info { fn fmt (& self , f : & mut Formatter) -> fmt :: Result { write ! (f , "{}" , self . os_type) ? ; if self . version != Version :: Unknown { write ! (f , " {}" , self . version) ? ; } if let Some (ref edition) = self . edition { write ! (f , " ({edition})") ? ; } if let Some (ref codename) = self . codename { write ! (f , " ({codename})") ? ; } write ! (f , " [{}]" , self . bitness) } }
};
}
