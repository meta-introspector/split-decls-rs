macro_rules! deps {
    () => {
        FrameDescriptionEntry!();
        EhFrame!();
        DebugFrame!();
        FrameTable!();
        CommonInformationEntry!();
        Writer!();
        Result!();
    };
}

macro_rules! impl_723 {
    () => {
        deps!();
        impl FrameTable { # [doc = " Add a CIE and return its id."] # [doc = ""] # [doc = " If the CIE already exists, then return the id of the existing CIE."] pub fn add_cie (& mut self , cie : CommonInformationEntry) -> CieId { let (index , _) = self . cies . insert_full (cie) ; CieId :: new (self . base_id , index) } # [doc = " The number of CIEs."] pub fn cie_count (& self) -> usize { self . cies . len () } # [doc = " Add a FDE."] # [doc = ""] # [doc = " Does not check for duplicates."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the CIE id is invalid."] pub fn add_fde (& mut self , cie : CieId , fde : FrameDescriptionEntry) { debug_assert_eq ! (self . base_id , cie . base_id) ; self . fdes . push ((cie , fde)) ; } # [doc = " The number of FDEs."] pub fn fde_count (& self) -> usize { self . fdes . len () } # [doc = " Write the frame table entries to the given `.debug_frame` section."] pub fn write_debug_frame < W : Writer > (& self , w : & mut DebugFrame < W >) -> Result < () > { self . write (& mut w . 0 , false) } # [doc = " Write the frame table entries to the given `.eh_frame` section."] pub fn write_eh_frame < W : Writer > (& self , w : & mut EhFrame < W >) -> Result < () > { self . write (& mut w . 0 , true) } fn write < W : Writer > (& self , w : & mut W , eh_frame : bool) -> Result < () > { let mut cie_offsets = vec ! [None ; self . cies . len ()] ; for (cie_id , fde) in & self . fdes { let cie_index = cie_id . index ; let cie = self . cies . get_index (cie_index) . unwrap () ; let cie_offset = match cie_offsets [cie_index] { Some (offset) => offset , None => { let offset = cie . write (w , eh_frame) ? ; cie_offsets [cie_index] = Some (offset) ; offset } } ; fde . write (w , eh_frame , cie_offset , cie) ? ; } Ok (()) } }
    };
}

impl_723!();