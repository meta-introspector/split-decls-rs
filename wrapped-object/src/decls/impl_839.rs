macro_rules! deps {
    () => {
        RelocationKind!();
        RelocationTarget!();
        Symbol!();
        FileHeader!();
        RelocationEncoding!();
        ReadRef!();
        Item!();
        RelocationFlags!();
        Relocation!();
        XcoffRelocationIterator!();
    };
}

macro_rules! impl_839 {
    () => {
        deps!();
        impl < 'data , 'file , Xcoff , R > Iterator for XcoffRelocationIterator < 'data , 'file , Xcoff , R > where Xcoff : FileHeader , R : ReadRef < 'data > , { type Item = (u64 , Relocation) ; fn next (& mut self) -> Option < Self :: Item > { self . relocations . next () . map (| relocation | { let r_rtype = relocation . r_rtype () ; let r_rsize = relocation . r_rsize () ; let flags = RelocationFlags :: Xcoff { r_rtype , r_rsize } ; let encoding = RelocationEncoding :: Generic ; let (kind , addend) = match r_rtype { xcoff :: R_POS | xcoff :: R_RL | xcoff :: R_RLA | xcoff :: R_BA | xcoff :: R_RBA | xcoff :: R_TLS => (RelocationKind :: Absolute , 0) , xcoff :: R_REL | xcoff :: R_BR | xcoff :: R_RBR => (RelocationKind :: Relative , - 4) , xcoff :: R_TOC | xcoff :: R_TOCL | xcoff :: R_TOCU => (RelocationKind :: Got , 0) , _ => (RelocationKind :: Unknown , 0) , } ; let size = (r_rsize & 0x3F) + 1 ; let target = RelocationTarget :: Symbol (relocation . symbol ()) ; (relocation . r_vaddr () . into () , Relocation { kind , encoding , size , target , addend , implicit_addend : true , flags , } ,) }) } }
    };
}

impl_839!()