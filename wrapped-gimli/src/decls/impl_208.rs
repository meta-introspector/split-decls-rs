macro_rules! deps {
    () => {
        UnwindTable!();
        BaseAddresses!();
        Error!();
        Format!();
        Result!();
        FrameDescriptionEntry!();
        PointerEncodingParameters!();
        UnwindContext!();
        Section!();
        Reader!();
        UnwindTableRow!();
        UnwindSection!();
        CommonInformationEntry!();
        AugmentationData!();
        UnwindContextStorage!();
    };
}

macro_rules! impl_208 {
    () => {
        deps!();
        impl < R : Reader > FrameDescriptionEntry < R > { fn parse_rest < Section , F > (offset : R :: Offset , length : R :: Offset , format : Format , cie_pointer : Section :: Offset , mut rest : R , section : & Section , bases : & BaseAddresses , mut get_cie : F ,) -> Result < FrameDescriptionEntry < R > > where Section : UnwindSection < R > , F : FnMut (& Section , & BaseAddresses , Section :: Offset) -> Result < CommonInformationEntry < R > > , { let cie = get_cie (section , bases , cie_pointer) ? ; let mut parameters = PointerEncodingParameters { bases : & bases . eh_frame , func_base : None , address_size : cie . address_size , section : section . section () , } ; let (initial_address , address_range) = Self :: parse_addresses (& mut rest , & cie , & parameters) ? ; parameters . func_base = Some (initial_address) ; let aug_data = if let Some (ref augmentation) = cie . augmentation { Some (AugmentationData :: parse (augmentation , & parameters , & mut rest ,) ?) } else { None } ; let entry = FrameDescriptionEntry { offset , length , format , cie , initial_address , address_range , augmentation : aug_data , instructions : rest , } ; Ok (entry) } fn parse_addresses (input : & mut R , cie : & CommonInformationEntry < R > , parameters : & PointerEncodingParameters < '_ , R > ,) -> Result < (u64 , u64) > { let encoding = cie . augmentation () . and_then (| a | a . fde_address_encoding) ; if let Some (encoding) = encoding { let initial_address = parse_encoded_pointer (encoding , parameters , input) ? . pointer () ; let address_range = parse_encoded_value (encoding , parameters , input) ? ; Ok ((initial_address , address_range)) } else { let initial_address = input . read_address (cie . address_size) ? ; let address_range = input . read_address (cie . address_size) ? ; Ok ((initial_address , address_range)) } } # [doc = " Return the table of unwind information for this FDE."] # [inline] pub fn rows < 'a , 'ctx , Section , S > (& self , section : & 'a Section , bases : & 'a BaseAddresses , ctx : & 'ctx mut UnwindContext < R :: Offset , S > ,) -> Result < UnwindTable < 'a , 'ctx , R , S > > where Section : UnwindSection < R > , S : UnwindContextStorage < R :: Offset > , { UnwindTable :: new (section , bases , ctx , self) } # [doc = " Find the frame unwind information for the given address."] # [doc = ""] # [doc = " If found, the unwind information is returned along with the reset"] # [doc = " context in the form `Ok((unwind_info, context))`. If not found,"] # [doc = " `Err(gimli::Error::NoUnwindInfoForAddress)` is returned. If parsing or"] # [doc = " CFI evaluation fails, the error is returned."] pub fn unwind_info_for_address < 'ctx , Section , S > (& self , section : & Section , bases : & BaseAddresses , ctx : & 'ctx mut UnwindContext < R :: Offset , S > , address : u64 ,) -> Result < & 'ctx UnwindTableRow < R :: Offset , S > > where Section : UnwindSection < R > , S : UnwindContextStorage < R :: Offset > , { let mut table = self . rows (section , bases , ctx) ? ; while let Some (row) = table . next_row () ? { if row . contains (address) { return Ok (table . ctx . row ()) ; } } Err (Error :: NoUnwindInfoForAddress) } }
    };
}

impl_208!();