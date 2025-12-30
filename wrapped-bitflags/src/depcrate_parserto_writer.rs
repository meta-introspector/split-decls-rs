// Generated macro for to_writer (function)
macro_rules! Depcrate_parserto_writer {
() => {
// Module: crate::parser
// Provides: {"to_writer"}
// Dependencies: {}
# [doc = "\nWrite a flags value as text.\n\nAny bits that aren't part of a contained flag will be formatted as a hex number.\n"] pub fn to_writer < B : Flags > (flags : & B , mut writer : impl Write) -> Result < () , fmt :: Error > where B :: Bits : WriteHex , { let mut first = true ; let mut iter = flags . iter_names () ; for (name , _) in & mut iter { if ! first { writer . write_str (" | ") ? ; } first = false ; writer . write_str (name) ? ; } let remaining = iter . remaining () . bits () ; if remaining != B :: Bits :: EMPTY { if ! first { writer . write_str (" | ") ? ; } writer . write_str ("0x") ? ; remaining . write_hex (writer) ? ; } fmt :: Result :: Ok (()) }
};
}
