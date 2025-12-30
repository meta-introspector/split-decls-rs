// Generated macro for impl_988 (impl)
macro_rules! Depcrate_machineimpl_988 {
() => {
// Module: crate::machine
// Provides: {"impl_988"}
// Dependencies: {}
impl interpret :: Provenance for Provenance { # [doc = " We use absolute addresses in the `offset` of a `StrictPointer`."] const OFFSET_IS_ADDR : bool = true ; # [doc = " Miri implements wildcard provenance."] const WILDCARD : Option < Self > = Some (Provenance :: Wildcard) ; fn get_alloc_id (self) -> Option < AllocId > { match self { Provenance :: Concrete { alloc_id , .. } => Some (alloc_id) , Provenance :: Wildcard => None , } } fn fmt (ptr : & interpret :: Pointer < Self > , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let (prov , addr) = ptr . into_raw_parts () ; write ! (f , "{:#x}" , addr . bytes ()) ? ; if f . alternate () { write ! (f , "{prov:#?}") ? ; } else { write ! (f , "{prov:?}") ? ; } Ok (()) } }
};
}
