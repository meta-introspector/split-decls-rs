macro_rules! deps {
    () => {
        AdditionalEntry!();
        SharedErrorSlot!();
    };
}

macro_rules! Stream {
    () => {
        deps!();
        # [doc = " A stream of entries that originate from a git tree and optionally from additional entries."] # [doc = ""] # [doc = " Note that a git tree is mandatory, but the empty tree can be used to effectively disable it."] pub struct Stream { read : utils :: Read , err : SharedErrorSlot , extra_entries : Option < std :: sync :: mpsc :: Sender < AdditionalEntry > > , # [doc = " `None` if currently held by an entry."] path_buf : Option < BString > , # [doc = " Another buffer to partially act like a buf-reader."] buf : Vec < u8 > , # [doc = " The offset into `buf` for entries being able to act like a buf reader."] pos : usize , # [doc = " The amount of bytes usable from `buf` (even though it always has a fixed size)"] filled : usize , }
    };
}

Stream!();