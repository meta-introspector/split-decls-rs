macro_rules! deps {
    () => {
        DwarfFileType!();
        Dwarf!();
    };
}

macro_rules! impl_264 {
    () => {
        deps!();
        impl < R : Clone > Dwarf < R > { # [doc = " Assuming `self` was loaded from a .dwo, take the appropriate"] # [doc = " sections from `parent` (which contains the skeleton unit for this"] # [doc = " dwo) such as `.debug_addr` and merge them into this `Dwarf`."] pub fn make_dwo (& mut self , parent : & Dwarf < R >) { self . file_type = DwarfFileType :: Dwo ; self . debug_addr = parent . debug_addr . clone () ; self . ranges . set_debug_ranges (parent . ranges . debug_ranges () . clone ()) ; self . sup . clone_from (& parent . sup) ; } }
    };
}

impl_264!();