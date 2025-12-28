macro_rules! deps {
    () => {
        FileHeader!();
        Bytes!();
        Endian!();
    };
}

macro_rules! NoteIterator {
    () => {
        deps!();
        # [doc = " An iterator over the notes in an ELF section or segment."] # [doc = ""] # [doc = " Returned [`ProgramHeader::notes`](super::ProgramHeader::notes)"] # [doc = " and [`SectionHeader::notes`](super::SectionHeader::notes)."] # [derive (Debug)] pub struct NoteIterator < 'data , Elf > where Elf : FileHeader , { endian : Elf :: Endian , align : usize , data : Bytes < 'data > , }
    };
}

NoteIterator!();