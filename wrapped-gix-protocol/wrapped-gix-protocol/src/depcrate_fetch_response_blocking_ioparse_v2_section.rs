// Generated macro for parse_v2_section (function)
macro_rules! Depcrate_fetch_response_blocking_ioparse_v2_section {
() => {
// Module: crate::fetch::response::blocking_io
// Provides: {"parse_v2_section"}
// Dependencies: {}
fn parse_v2_section < 'a , T > (line : & mut String , reader : & mut impl ExtendedBufRead < 'a > , res : & mut Vec < T > , parse : impl Fn (& str) -> Result < T , response :: Error > ,) -> Result < bool , response :: Error > { line . clear () ; while reader . readline_str (line) ? != 0 { res . push (parse (line) ?) ; line . clear () ; } Ok (if reader . stopped_at () == Some (MessageKind :: Delimiter) { reader . reset (Protocol :: V2) ; false } else { true }) }
};
}
