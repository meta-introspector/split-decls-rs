// Generated macro for impl_39 (impl)
macro_rules! Depcrate_keysimpl_39 {
() => {
// Module: crate::keys
// Provides: {"impl_39"}
// Dependencies: {}
impl Name { # [doc = " Returns the [`Mib`] of `self`."] pub fn mib < T : MibArg > (& self) -> Result < Mib < T > > { let mut mib : Mib < T > = Mib :: default () ; raw :: name_to_mib (& self . 0 , mib . 0 . as_mut ()) ? ; Ok (mib) } # [doc = " Returns the [`MibStr`] of `self` which is a key whose value is a string."] pub fn mib_str < T : MibArg > (& self) -> Result < MibStr < T > > { assert ! (self . value_type_str () , "key \"{}\" does not refer to a string" , self) ; let mut mib : MibStr < T > = MibStr :: default () ; raw :: name_to_mib (& self . 0 , mib . 0 . as_mut ()) ? ; Ok (mib) } # [doc = " Returns `true` if `self` is a key in the _MALLCTL NAMESPCE_ referring to"] # [doc = " a null-terminated string."] pub fn value_type_str (& self) -> bool { let name = self . 0 . split_at (self . 0 . len () - 1) . 0 ; if name . is_empty () { return false ; } debug_assert_ne ! (* name . last () . unwrap () , b'\0') ; match name { b"version" | b"config.malloc_conf" | b"opt.metadata_thp" | b"opt.dss" | b"opt.percpu_arena" | b"opt.stats_print_opts" | b"opt.junk" | b"opt.thp" | b"opt.prof_prefix" | b"thread.prof.name" | b"prof.dump" => true , v if v . starts_with (b"arena.") && v . ends_with (b".dss") => true , v if v . starts_with (b"stats.arenas.") && v . ends_with (b".dss") => { true } _ => false , } } # [doc = " Returns the name as null-terminated byte-string."] pub fn as_bytes (& self) -> & 'static [u8] { unsafe { & * (self as * const Self as * const [u8]) } } }
};
}
