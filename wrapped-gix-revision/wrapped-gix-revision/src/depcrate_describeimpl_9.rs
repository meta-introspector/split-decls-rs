// Generated macro for impl_9 (impl)
macro_rules! Depcrate_describeimpl_9 {
() => {
// Module: crate::describe
// Provides: {"impl_9"}
// Dependencies: {}
impl Display for Format < '_ > { fn fmt (& self , f : & mut Formatter < '_ >) -> std :: fmt :: Result { if let Some (name) = self . name . as_deref () { if ! self . long && self . is_exact_match () { name . fmt (f) ? ; } else { write ! (f , "{}-{}-g{}" , name , self . depth , self . id . to_hex_with_len (self . hex_len)) ? ; } } else { self . id . to_hex_with_len (self . hex_len) . fmt (f) ? ; } if let Some (suffix) = & self . dirty_suffix { write ! (f , "-{suffix}") ? ; } Ok (()) } }
};
}
