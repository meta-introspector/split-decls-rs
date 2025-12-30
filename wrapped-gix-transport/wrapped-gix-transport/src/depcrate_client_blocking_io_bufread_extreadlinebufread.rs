// Generated macro for ReadlineBufRead (trait)
macro_rules! Depcrate_client_blocking_io_bufread_extReadlineBufRead {
() => {
// Module: crate::client::blocking_io::bufread_ext
// Provides: {"ReadlineBufRead"}
// Dependencies: {}
# [doc = " This trait exists to get a version of a `gix_packetline::Provider` without type parameters,"] # [doc = " but leave support for reading lines directly without forcing them through `String`."] # [doc = ""] # [doc = " For the sake of usability, it also implements [`std::io::BufRead`] making it trivial to"] # [doc = " read pack files while keeping open the option to read individual lines with low overhead."] pub trait ReadlineBufRead : io :: BufRead { # [doc = " Read a packet line into the internal buffer and return it."] # [doc = ""] # [doc = " Returns `None` if the end of iteration is reached because of one of the following:"] # [doc = ""] # [doc = "  * natural EOF"] # [doc = "  * ERR packet line encountered"] # [doc = "  * A `delimiter` packet line encountered"] fn readline (& mut self ,) -> Option < io :: Result < Result < gix_packetline :: PacketLineRef < '_ > , gix_packetline :: decode :: Error > > > ; # [doc = " Read a line similar to `BufRead::read_line()`, but assure it doesn't try to find newlines"] # [doc = " which might concatenate multiple distinct packet lines."] # [doc = ""] # [doc = " Making this a trait method allows to handle differences between async and blocking."] fn readline_str (& mut self , line : & mut String) -> io :: Result < usize > ; }
};
}
