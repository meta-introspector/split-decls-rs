macro_rules! deps {
    () => {
        PointerEncodingParameters!();
        Pointer!();
        Error!();
        Result!();
        Reader!();
        EhHdrTableIter!();
    };
}

macro_rules! impl_171 {
    () => {
        deps!();
        impl < 'a , 'bases , R : Reader > EhHdrTableIter < 'a , 'bases , R > { # [doc = " Yield the next entry in the `EhHdrTableIter`."] pub fn next (& mut self) -> Result < Option < (Pointer , Pointer) > > { if self . remain == 0 { return Ok (None) ; } let parameters = PointerEncodingParameters { bases : & self . bases . eh_frame_hdr , func_base : None , address_size : self . hdr . address_size , section : & self . hdr . section , } ; self . remain -= 1 ; let from = parse_encoded_pointer (self . hdr . table_enc , & parameters , & mut self . table) ? ; let to = parse_encoded_pointer (self . hdr . table_enc , & parameters , & mut self . table) ? ; Ok (Some ((from , to))) } # [doc = " Yield the nth entry in the `EhHdrTableIter`"] pub fn nth (& mut self , n : usize) -> Result < Option < (Pointer , Pointer) > > { use core :: convert :: TryFrom ; let size = match self . hdr . table_enc . format () { constants :: DW_EH_PE_uleb128 | constants :: DW_EH_PE_sleb128 => { return Err (Error :: VariableLengthSearchTable) ; } constants :: DW_EH_PE_sdata2 | constants :: DW_EH_PE_udata2 => 2 , constants :: DW_EH_PE_sdata4 | constants :: DW_EH_PE_udata4 => 4 , constants :: DW_EH_PE_sdata8 | constants :: DW_EH_PE_udata8 => 8 , _ => return Err (Error :: UnknownPointerEncoding (self . hdr . table_enc)) , } ; let row_size = size * 2 ; let n = u64 :: try_from (n) . map_err (| _ | Error :: UnsupportedOffset) ? ; self . remain = self . remain . saturating_sub (n) ; self . table . skip (R :: Offset :: from_u64 (n * row_size) ?) ? ; self . next () } }
    };
}

impl_171!()