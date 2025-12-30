// Generated macro for impl_266 (impl)
macro_rules! Depcrate_progress_logimpl_266 {
() => {
// Module: crate::progress::log
// Provides: {"impl_266"}
// Dependencies: {}
impl Progress for Log { fn init (& mut self , max : Option < Step > , unit : Option < Unit >) { self . max = max ; self . unit = unit ; } fn unit (& self) -> Option < Unit > { self . unit . clone () } fn max (& self) -> Option < Step > { self . max } fn set_max (& mut self , max : Option < Step >) -> Option < Step > { let prev = self . max ; self . max = max ; prev } fn set_name (& mut self , name : String) { self . name = self . name . split ("::") . next () . map (| parent | format ! ("{}{}{}" , parent . to_owned () , SEP , name)) . unwrap_or (name) ; } fn name (& self) -> Option < String > { self . name . split (SEP) . nth (1) . map (ToOwned :: to_owned) } fn id (& self) -> Id { self . id } fn message (& self , level : MessageLevel , message : String) { match level { MessageLevel :: Info => log :: info ! ("ℹ{} → {}" , self . name , message) , MessageLevel :: Failure => log :: error ! ("𐄂{} → {}" , self . name , message) , MessageLevel :: Success => log :: info ! ("✓{} → {}" , self . name , message) , } } }
};
}
