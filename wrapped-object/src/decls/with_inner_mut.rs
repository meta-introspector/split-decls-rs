macro_rules! deps {
    () => {
        MachO32!();
        MachO64!();
    };
}

macro_rules! with_inner_mut {
    () => {
        deps!();
        macro_rules ! with_inner_mut { ($ inner : expr , $ enum : ident , | $ var : ident | $ body : expr) => { match $ inner { # [cfg (feature = "coff")] $ enum :: Coff (ref mut $ var) => $ body , # [cfg (feature = "coff")] $ enum :: CoffBig (ref mut $ var) => $ body , # [cfg (feature = "elf")] $ enum :: Elf32 (ref mut $ var) => $ body , # [cfg (feature = "elf")] $ enum :: Elf64 (ref mut $ var) => $ body , # [cfg (feature = "macho")] $ enum :: MachO32 (ref mut $ var) => $ body , # [cfg (feature = "macho")] $ enum :: MachO64 (ref mut $ var) => $ body , # [cfg (feature = "pe")] $ enum :: Pe32 (ref mut $ var) => $ body , # [cfg (feature = "pe")] $ enum :: Pe64 (ref mut $ var) => $ body , # [cfg (feature = "wasm")] $ enum :: Wasm (ref mut $ var) => $ body , # [cfg (feature = "xcoff")] $ enum :: Xcoff32 (ref mut $ var) => $ body , # [cfg (feature = "xcoff")] $ enum :: Xcoff64 (ref mut $ var) => $ body , } } ; }
    };
}

with_inner_mut!()