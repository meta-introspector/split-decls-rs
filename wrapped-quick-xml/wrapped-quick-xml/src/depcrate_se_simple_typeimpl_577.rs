// Generated macro for impl_577 (impl)
macro_rules! Depcrate_se_simple_typeimpl_577 {
() => {
// Module: crate::se::simple_type
// Provides: {"impl_577"}
// Dependencies: {}
impl < W : Write > SerializeSeq for SimpleSeq < W > { type Ok = W ; type Error = SeError ; fn serialize_element < T > (& mut self , value : & T) -> Result < () , Self :: Error > where T : ? Sized + Serialize , { if value . serialize (AtomicSerializer { writer : & mut self . writer , target : self . target , level : self . level , write_delimiter : ! self . is_empty , }) ? { self . is_empty = false ; } Ok (()) } # [inline] fn end (mut self) -> Result < Self :: Ok , Self :: Error > { if let QuoteTarget :: CData = self . target { self . writer . write_str ("]]>") ? ; } Ok (self . writer) } }
};
}
