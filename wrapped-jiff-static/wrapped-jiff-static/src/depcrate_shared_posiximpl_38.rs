// Generated macro for impl_38 (impl)
macro_rules! Depcrate_shared_posiximpl_38 {
() => {
// Module: crate::shared::posix
// Provides: {"impl_38"}
// Dependencies: {}
impl < ABBREV : AsRef < str > > PosixDst < ABBREV > { fn display (& self , std_offset : & PosixOffset , f : & mut core :: fmt :: Formatter ,) -> core :: fmt :: Result { write ! (f , "{}" , AbbreviationDisplay (self . abbrev . as_ref ())) ? ; let default = PosixOffset { second : std_offset . second + 3600 } ; if self . offset != default { write ! (f , "{}" , self . offset) ? ; } write ! (f , ",{}" , self . rule) ? ; Ok (()) } }
};
}
