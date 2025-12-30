// Generated macro for impl_965 (impl)
macro_rules! Depcrate_ir_memflagsimpl_965 {
() => {
// Module: crate::ir::memflags
// Provides: {"impl_965"}
// Dependencies: {}
impl fmt :: Display for MemFlags { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match self . trap_code () { None => write ! (f , " notrap") ? , Some (TrapCode :: HEAP_OUT_OF_BOUNDS) => { } Some (t) => write ! (f , " {t}") ? , } if self . aligned () { write ! (f , " aligned") ? ; } if self . readonly () { write ! (f , " readonly") ? ; } if self . can_move () { write ! (f , " can_move") ? ; } if self . read_bit (BIT_BIG_ENDIAN) { write ! (f , " big") ? ; } if self . read_bit (BIT_LITTLE_ENDIAN) { write ! (f , " little") ? ; } if self . checked () { write ! (f , " checked") ? ; } match self . alias_region () { None => { } Some (AliasRegion :: Heap) => write ! (f , " heap") ? , Some (AliasRegion :: Table) => write ! (f , " table") ? , Some (AliasRegion :: Vmctx) => write ! (f , " vmctx") ? , } Ok (()) } }
};
}
