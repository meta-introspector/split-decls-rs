// Generated macro for derive_header (macro)
macro_rules! Depcrate_utilderive_header {
() => {
// Module: crate::util
// Provides: {"derive_header"}
// Dependencies: {}
macro_rules ! derive_header { ($ type : ident (_) , name : $ name : ident) => { impl crate :: Header for $ type { fn name () -> &'static :: http :: header :: HeaderName { &:: http :: header ::$ name } fn decode <'i , I > (values : & mut I) -> Result < Self , crate :: Error > where I : Iterator < Item = &'i :: http :: header :: HeaderValue >, { crate :: util :: TryFromValues :: try_from_values (values) . map ($ type) } fn encode < E : Extend < crate :: HeaderValue >> (& self , values : & mut E) { values . extend (:: std :: iter :: once ((& self . 0) . into ())) ; } } } ; }
};
}
