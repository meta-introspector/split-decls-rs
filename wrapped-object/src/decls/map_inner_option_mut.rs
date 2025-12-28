macro_rules! deps {
    () => {
        MachO64!();
        MachO32!();
    };
}

macro_rules! map_inner_option_mut {
    () => {
        deps!();
        macro_rules ! map_inner_option_mut { ($ inner : expr , $ from : ident , $ to : ident , | $ var : ident | $ body : expr) => { match $ inner { # [cfg (feature = "coff")] $ from :: Coff (ref mut $ var) => $ body . map ($ to :: Coff) , # [cfg (feature = "coff")] $ from :: CoffBig (ref mut $ var) => $ body . map ($ to :: CoffBig) , # [cfg (feature = "elf")] $ from :: Elf32 (ref mut $ var) => $ body . map ($ to :: Elf32) , # [cfg (feature = "elf")] $ from :: Elf64 (ref mut $ var) => $ body . map ($ to :: Elf64) , # [cfg (feature = "macho")] $ from :: MachO32 (ref mut $ var) => $ body . map ($ to :: MachO32) , # [cfg (feature = "macho")] $ from :: MachO64 (ref mut $ var) => $ body . map ($ to :: MachO64) , # [cfg (feature = "pe")] $ from :: Pe32 (ref mut $ var) => $ body . map ($ to :: Pe32) , # [cfg (feature = "pe")] $ from :: Pe64 (ref mut $ var) => $ body . map ($ to :: Pe64) , # [cfg (feature = "wasm")] $ from :: Wasm (ref mut $ var) => $ body . map ($ to :: Wasm) , # [cfg (feature = "xcoff")] $ from :: Xcoff32 (ref mut $ var) => $ body . map ($ to :: Xcoff32) , # [cfg (feature = "xcoff")] $ from :: Xcoff64 (ref mut $ var) => $ body . map ($ to :: Xcoff64) , } } ; }
    };
}

map_inner_option_mut!();