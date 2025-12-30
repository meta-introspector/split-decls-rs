// Generated macro for impl_408 (impl)
macro_rules! Depcrate_provider_fields_symbolsimpl_408 {
() => {
// Module: crate::provider::fields::symbols
// Provides: {"impl_408"}
// Dependencies: {}
impl TryFrom < char > for FieldSymbol { type Error = SymbolError ; fn try_from (ch : char) -> Result < Self , Self :: Error > { if ! ch . is_ascii_alphanumeric () { return Err (SymbolError :: Invalid (ch as u8)) ; } (if ch == 'G' { Ok (Self :: Era) } else { Err (SymbolError :: Unknown (ch)) }) . or_else (| _ | Year :: try_from (ch) . map (Self :: Year)) . or_else (| _ | Month :: try_from (ch) . map (Self :: Month)) . or_else (| _ | Week :: try_from (ch) . map (Self :: Week)) . or_else (| _ | Day :: try_from (ch) . map (Self :: Day)) . or_else (| _ | Weekday :: try_from (ch) . map (Self :: Weekday)) . or_else (| _ | DayPeriod :: try_from (ch) . map (Self :: DayPeriod)) . or_else (| _ | Hour :: try_from (ch) . map (Self :: Hour)) . or ({ if ch == 'm' { Ok (Self :: Minute) } else { Err (SymbolError :: Unknown (ch)) } }) . or_else (| _ | Second :: try_from (ch) . map (Self :: Second)) . or_else (| _ | TimeZone :: try_from (ch) . map (Self :: TimeZone)) } }
};
}
