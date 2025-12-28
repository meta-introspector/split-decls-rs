macro_rules! deps {
    () => {
        TraitEnvironment!();
        HirDatabase!();
        MemoryMap!();
    };
}

macro_rules! intern_const_ref {
    () => {
        deps!();
        # [doc = " Interns a constant scalar with the given type"] pub fn intern_const_ref < 'a > (db : & 'a dyn HirDatabase , value : & LiteralConstRef , ty : Ty < 'a > , krate : Crate ,) -> Const < 'a > { let interner = DbInterner :: new_with (db , Some (krate) , None) ; let layout = db . layout_of_ty (ty , TraitEnvironment :: empty (krate)) ; let kind = match value { LiteralConstRef :: Int (i) => { let size = layout . map (| it | it . size . bytes_usize ()) . unwrap_or (16) ; rustc_type_ir :: ConstKind :: Value (ValueConst :: new (ty , ConstBytes { memory : i . to_le_bytes () [0 .. size] . into () , memory_map : MemoryMap :: default () , } ,)) } LiteralConstRef :: UInt (i) => { let size = layout . map (| it | it . size . bytes_usize ()) . unwrap_or (16) ; rustc_type_ir :: ConstKind :: Value (ValueConst :: new (ty , ConstBytes { memory : i . to_le_bytes () [0 .. size] . into () , memory_map : MemoryMap :: default () , } ,)) } LiteralConstRef :: Bool (b) => rustc_type_ir :: ConstKind :: Value (ValueConst :: new (ty , ConstBytes { memory : Box :: new ([* b as u8]) , memory_map : MemoryMap :: default () } ,)) , LiteralConstRef :: Char (c) => rustc_type_ir :: ConstKind :: Value (ValueConst :: new (ty , ConstBytes { memory : (* c as u32) . to_le_bytes () . into () , memory_map : MemoryMap :: default () , } ,)) , LiteralConstRef :: Unknown => rustc_type_ir :: ConstKind :: Error (ErrorGuaranteed) , } ; Const :: new (interner , kind) }
    };
}

intern_const_ref!();