macro_rules! deps {
    () => {
        EhHdrTable!();
        Error!();
        EhFrame!();
        ParsedEhFrameHdr!();
        PointerEncodingParameters!();
        EndianSlice!();
        FrameDescriptionEntry!();
        Result!();
        CommonInformationEntry!();
        Pointer!();
        EhHdrTableIter!();
        EhFrameOffset!();
        BaseAddresses!();
        UnwindContextStorage!();
        Reader!();
        UnwindSection!();
        UnwindContext!();
        UnwindTableRow!();
    };
}

macro_rules! impl_174 {
    () => {
        deps!();
        impl < 'a , R : Reader + 'a > EhHdrTable < 'a , R > { # [doc = " Return an iterator that can walk the `.eh_frame_hdr` table."] # [doc = ""] # [doc = " Each table entry consists of a tuple containing an `initial_location` and `address`."] # [doc = " The `initial location` represents the first address that the targeted FDE"] # [doc = " is able to decode. The `address` is the address of the FDE in the `.eh_frame` section."] # [doc = " The `address` can be converted with `EhHdrTable::pointer_to_offset` and `EhFrame::fde_from_offset` to an FDE."] pub fn iter < 'bases > (& self , bases : & 'bases BaseAddresses) -> EhHdrTableIter < '_ , 'bases , R > { EhHdrTableIter { hdr : self . hdr , bases , remain : self . hdr . fde_count , table : self . hdr . table . clone () , } } # [doc = " *Probably* returns a pointer to the FDE for the given address."] # [doc = ""] # [doc = " This performs a binary search, so if there is no FDE for the given address,"] # [doc = " this function **will** return a pointer to any other FDE that's close by."] # [doc = ""] # [doc = " To be sure, you **must** call `contains` on the FDE."] pub fn lookup (& self , address : u64 , bases : & BaseAddresses) -> Result < Pointer > { let size = match self . hdr . table_enc . format () { constants :: DW_EH_PE_uleb128 | constants :: DW_EH_PE_sleb128 => { return Err (Error :: VariableLengthSearchTable) ; } constants :: DW_EH_PE_sdata2 | constants :: DW_EH_PE_udata2 => 2 , constants :: DW_EH_PE_sdata4 | constants :: DW_EH_PE_udata4 => 4 , constants :: DW_EH_PE_sdata8 | constants :: DW_EH_PE_udata8 => 8 , _ => return Err (Error :: UnknownPointerEncoding (self . hdr . table_enc)) , } ; let row_size = size * 2 ; let mut len = self . hdr . fde_count ; let mut reader = self . hdr . table . clone () ; let parameters = PointerEncodingParameters { bases : & bases . eh_frame_hdr , func_base : None , address_size : self . hdr . address_size , section : & self . hdr . section , } ; while len > 1 { let head = reader . split (R :: Offset :: from_u64 ((len / 2) * row_size) ?) ? ; let tail = reader . clone () ; let pivot = parse_encoded_pointer (self . hdr . table_enc , & parameters , & mut reader) ? . direct () ? ; match pivot . cmp (& address) { Ordering :: Equal => { reader = tail ; break ; } Ordering :: Less => { reader = tail ; len = len - (len / 2) ; } Ordering :: Greater => { reader = head ; len /= 2 ; } } } reader . skip (R :: Offset :: from_u64 (size) ?) ? ; parse_encoded_pointer (self . hdr . table_enc , & parameters , & mut reader) } # [doc = " Convert a `Pointer` to a section offset."] # [doc = ""] # [doc = " This does not support indirect pointers."] pub fn pointer_to_offset (& self , ptr : Pointer) -> Result < EhFrameOffset < R :: Offset > > { let ptr = ptr . direct () ? ; let eh_frame_ptr = self . hdr . eh_frame_ptr () . direct () ? ; R :: Offset :: from_u64 (ptr - eh_frame_ptr) . map (EhFrameOffset) } # [doc = " Returns a parsed FDE for the given address, or `NoUnwindInfoForAddress`"] # [doc = " if there are none."] # [doc = ""] # [doc = " You must provide a function to get its associated CIE. See"] # [doc = " `PartialFrameDescriptionEntry::parse` for more information."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use gimli::{BaseAddresses, EhFrame, ParsedEhFrameHdr, EndianSlice, NativeEndian, Error, UnwindSection};"] # [doc = " # fn foo() -> Result<(), Error> {"] # [doc = " # let eh_frame: EhFrame<EndianSlice<NativeEndian>> = unreachable!();"] # [doc = " # let eh_frame_hdr: ParsedEhFrameHdr<EndianSlice<NativeEndian>> = unimplemented!();"] # [doc = " # let addr = 0;"] # [doc = " # let bases = unimplemented!();"] # [doc = " let table = eh_frame_hdr.table().unwrap();"] # [doc = " let fde = table.fde_for_address(&eh_frame, &bases, addr, EhFrame::cie_from_offset)?;"] # [doc = " # Ok(())"] # [doc = " # }"] # [doc = " ```"] pub fn fde_for_address < F > (& self , frame : & EhFrame < R > , bases : & BaseAddresses , address : u64 , get_cie : F ,) -> Result < FrameDescriptionEntry < R > > where F : FnMut (& EhFrame < R > , & BaseAddresses , EhFrameOffset < R :: Offset > ,) -> Result < CommonInformationEntry < R > > , { let fdeptr = self . lookup (address , bases) ? ; let offset = self . pointer_to_offset (fdeptr) ? ; let entry = frame . fde_from_offset (bases , offset , get_cie) ? ; if entry . contains (address) { Ok (entry) } else { Err (Error :: NoUnwindInfoForAddress) } } # [doc = " Returns the frame unwind information for the given address,"] # [doc = " or `NoUnwindInfoForAddress` if there are none."] # [doc = ""] # [doc = " You must provide a function to get the associated CIE. See"] # [doc = " `PartialFrameDescriptionEntry::parse` for more information."] pub fn unwind_info_for_address < 'ctx , F , S > (& self , frame : & EhFrame < R > , bases : & BaseAddresses , ctx : & 'ctx mut UnwindContext < R :: Offset , S > , address : u64 , get_cie : F ,) -> Result < & 'ctx UnwindTableRow < R :: Offset , S > > where F : FnMut (& EhFrame < R > , & BaseAddresses , EhFrameOffset < R :: Offset > ,) -> Result < CommonInformationEntry < R > > , S : UnwindContextStorage < R :: Offset > , { let fde = self . fde_for_address (frame , bases , address , get_cie) ? ; fde . unwind_info_for_address (frame , bases , ctx , address) } }
    };
}

impl_174!();