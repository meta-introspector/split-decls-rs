// Generated macro for impl_125 (impl)
macro_rules! Depcrate_interact_sessionimpl_125 {
() => {
// Module: crate::interact::session
// Provides: {"impl_125"}
// Dependencies: {}
impl < S , I , O , C > std :: fmt :: Debug for InteractSession < S , I , O , C > where S : std :: fmt :: Debug , I : std :: fmt :: Debug , O : std :: fmt :: Debug , C : std :: fmt :: Debug , { # [rustfmt :: skip] fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { let mut s = f . debug_struct ("InteractSession") ; let _ = s . field ("session" , & self . session) . field ("input" , & self . input) . field ("output" , & self . output) . field ("escape_character" , & self . escape_character) ; # [cfg (unix)] { let _ = s . field ("status" , & self . status) ; } let _ = s . field ("state" , & std :: ptr :: addr_of ! (self . opts . state)) . field ("opts:on_idle" , & get_pointer (& self . opts . idle_action)) . field ("opts:on_input" , & get_pointer (& self . opts . input_action)) . field ("opts:on_output" , & get_pointer (& self . opts . output_action)) . field ("opts:input_filter" , & get_pointer (& self . opts . input_filter)) . field ("opts:output_filter" , & get_pointer (& self . opts . output_filter)) ; s . finish () } }
};
}
