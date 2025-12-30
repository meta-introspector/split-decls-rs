// Generated macro for impl_152 (impl)
macro_rules! Depcrate_stream_bufferedimpl_152 {
() => {
// Module: crate::stream::buffered
// Provides: {"impl_152"}
// Dependencies: {}
impl < Input > StreamOnce for Stream < Input > where Input : StreamOnce + Positioned , Input :: Token : Clone , { type Token = Input :: Token ; type Range = Input :: Range ; type Position = Input :: Position ; type Error = Input :: Error ; # [inline] fn uncons (& mut self) -> Result < Input :: Token , StreamErrorFor < Self > > { if self . offset >= self . buffer_offset { let position = self . iter . position () ; let token = self . iter . uncons () ? ; self . buffer_offset += 1 ; if self . buffer . len () == self . buffer . capacity () { self . buffer . pop_front () ; } self . buffer . push_back ((token . clone () , position)) ; self . offset += 1 ; Ok (token) } else if self . offset < self . buffer_offset - self . buffer . len () { Err (StreamError :: message_static_message ("Backtracked to far")) } else { let value = self . buffer [self . buffer . len () - (self . buffer_offset - self . offset)] . 0 . clone () ; self . offset += 1 ; Ok (value) } } fn is_partial (& self) -> bool { self . iter . is_partial () } }
};
}
