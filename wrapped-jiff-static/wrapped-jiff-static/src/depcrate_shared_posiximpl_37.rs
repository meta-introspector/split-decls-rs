// Generated macro for impl_37 (impl)
macro_rules! Depcrate_shared_posiximpl_37 {
() => {
// Module: crate::shared::posix
// Provides: {"impl_37"}
// Dependencies: {}
impl < ABBREV : AsRef < str > > core :: fmt :: Display for PosixTimeZone < ABBREV > { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { write ! (f , "{}{}" , AbbreviationDisplay (self . std_abbrev . as_ref ()) , self . std_offset) ? ; if let Some (ref dst) = self . dst { dst . display (& self . std_offset , f) ? ; } Ok (()) } }
};
}
