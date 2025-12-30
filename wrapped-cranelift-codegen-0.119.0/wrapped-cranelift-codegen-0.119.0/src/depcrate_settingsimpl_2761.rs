// Generated macro for impl_2761 (impl)
macro_rules! Depcrate_settingsimpl_2761 {
() => {
// Module: crate::settings
// Provides: {"impl_2761"}
// Dependencies: {}
impl fmt :: Display for Value { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { if let Some (enum_variant) = self . as_enum () { write ! (f , "{}={}" , self . name , enum_variant) } else if let Some (num) = self . as_num () { write ! (f , "{}={}" , self . name , num) } else if let Some (b) = self . as_bool () { if b { write ! (f , "{}=1" , self . name) } else { write ! (f , "{}=0" , self . name) } } else { unreachable ! () } } }
};
}
