// Generated macro for parse_v2_section (function)
macro_rules! Depcrate_fetch_response_async_ioparse_v2_section {
() => {
// Module: crate::fetch::response::async_io
// Provides: {"parse_v2_section"}
// Dependencies: {}
async fn parse_v2_section < T > (line : & mut String , reader : & mut (impl ExtendedBufRead < '_ > + Unpin) , res : & mut Vec < T > , parse : impl Fn (& str) -> Result < T , response :: Error > ,) -> Result < bool , response :: Error > { line . clear () ; while reader . readline_str (line) . await ? != 0 { res . push (parse (line) ?) ; line . clear () ; } Ok (if reader . stopped_at () == Some (client :: MessageKind :: Delimiter) { reader . reset (Protocol :: V2) ; false } else { true }) }
};
}
