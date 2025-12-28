macro_rules! deps {
    () => {
        WriteHex!();
        Flags!();
        Bits!();
    };
}

macro_rules! to_writer_truncate {
    () => {
        deps!();
        # [doc = "\nWrite a flags value as text, ignoring any unknown bits.\n"] pub fn to_writer_truncate < B : Flags > (flags : & B , writer : impl Write) -> Result < () , fmt :: Error > where B :: Bits : WriteHex , { to_writer (& B :: from_bits_truncate (flags . bits ()) , writer) }
    };
}

to_writer_truncate!()