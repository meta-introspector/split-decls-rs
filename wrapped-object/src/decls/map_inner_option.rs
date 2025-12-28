macro_rules! deps {
    () => {
        MachO64!();
        MachO32!();
        Result!();
    };
}

macro_rules! map_inner_option {
    () => {
        deps!();
        # [doc = " Like `map_inner!`, but the result is a Result or Option."] macro_rules ! map_inner_option { ($ inner : expr , $ from : ident , $ to : ident , | $ var : ident | $ body : expr) => { match $ inner { # [cfg (feature = "coff")] $ from :: Coff (ref $ var) => $ body . map ($ to :: Coff) , # [cfg (feature = "coff")] $ from :: CoffBig (ref $ var) => $ body . map ($ to :: CoffBig) , # [cfg (feature = "elf")] $ from :: Elf32 (ref $ var) => $ body . map ($ to :: Elf32) , # [cfg (feature = "elf")] $ from :: Elf64 (ref $ var) => $ body . map ($ to :: Elf64) , # [cfg (feature = "macho")] $ from :: MachO32 (ref $ var) => $ body . map ($ to :: MachO32) , # [cfg (feature = "macho")] $ from :: MachO64 (ref $ var) => $ body . map ($ to :: MachO64) , # [cfg (feature = "pe")] $ from :: Pe32 (ref $ var) => $ body . map ($ to :: Pe32) , # [cfg (feature = "pe")] $ from :: Pe64 (ref $ var) => $ body . map ($ to :: Pe64) , # [cfg (feature = "wasm")] $ from :: Wasm (ref $ var) => $ body . map ($ to :: Wasm) , # [cfg (feature = "xcoff")] $ from :: Xcoff32 (ref $ var) => $ body . map ($ to :: Xcoff32) , # [cfg (feature = "xcoff")] $ from :: Xcoff64 (ref $ var) => $ body . map ($ to :: Xcoff64) , } } ; }
    };
}

map_inner_option!();