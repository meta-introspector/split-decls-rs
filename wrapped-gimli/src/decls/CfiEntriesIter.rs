macro_rules! deps {
    () => {
        EndianSlice!();
        UnwindSection!();
        BaseAddresses!();
        Section!();
        Reader!();
        EhFrame!();
    };
}

macro_rules! CfiEntriesIter {
    () => {
        deps!();
        # [doc = " An iterator over CIE and FDE entries in a `.debug_frame` or `.eh_frame`"] # [doc = " section."] # [doc = ""] # [doc = " Some pointers may be encoded relative to various base addresses. Use the"] # [doc = " [`BaseAddresses`](./struct.BaseAddresses.html) parameter to provide them. By"] # [doc = " default, none are provided. If a relative pointer is encountered for a base"] # [doc = " address that is unknown, an `Err` will be returned and iteration will abort."] # [doc = ""] # [doc = " Can be [used with"] # [doc = " `FallibleIterator`](./index.html#using-with-fallibleiterator)."] # [doc = ""] # [doc = " ```"] # [doc = " use gimli::{BaseAddresses, EhFrame, EndianSlice, NativeEndian, UnwindSection};"] # [doc = ""] # [doc = " # fn foo() -> gimli::Result<()> {"] # [doc = " # let read_eh_frame_somehow = || unimplemented!();"] # [doc = " let eh_frame = EhFrame::new(read_eh_frame_somehow(), NativeEndian);"] # [doc = ""] # [doc = " # let address_of_eh_frame_hdr_section_in_memory = unimplemented!();"] # [doc = " # let address_of_eh_frame_section_in_memory = unimplemented!();"] # [doc = " # let address_of_text_section_in_memory = unimplemented!();"] # [doc = " # let address_of_got_section_in_memory = unimplemented!();"] # [doc = " # let address_of_the_start_of_current_func = unimplemented!();"] # [doc = " // Provide base addresses for relative pointers."] # [doc = " let bases = BaseAddresses::default()"] # [doc = "     .set_eh_frame_hdr(address_of_eh_frame_hdr_section_in_memory)"] # [doc = "     .set_eh_frame(address_of_eh_frame_section_in_memory)"] # [doc = "     .set_text(address_of_text_section_in_memory)"] # [doc = "     .set_got(address_of_got_section_in_memory);"] # [doc = ""] # [doc = " let mut entries = eh_frame.entries(&bases);"] # [doc = ""] # [doc = " # let do_stuff_with = |_| unimplemented!();"] # [doc = " while let Some(entry) = entries.next()? {"] # [doc = "     do_stuff_with(entry)"] # [doc = " }"] # [doc = " # unreachable!()"] # [doc = " # }"] # [doc = " ```"] # [derive (Clone , Debug)] pub struct CfiEntriesIter < 'bases , Section , R > where R : Reader , Section : UnwindSection < R > , { section : Section , bases : & 'bases BaseAddresses , input : R , }
    };
}

CfiEntriesIter!()