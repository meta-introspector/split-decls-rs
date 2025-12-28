macro_rules! deps {
    () => {
        MachO32!();
        MachO64!();
    };
}

macro_rules! map_inner {
    () => {
        deps!();
        # [doc = " Like `with_inner!`, but wraps the result in another enum."] macro_rules ! map_inner { ($ inner : expr , $ from : ident , $ to : ident , | $ var : ident | $ body : expr) => { match $ inner { # [cfg (feature = "coff")] $ from :: Coff (ref $ var) => $ to :: Coff ($ body) , # [cfg (feature = "coff")] $ from :: CoffBig (ref $ var) => $ to :: CoffBig ($ body) , # [cfg (feature = "elf")] $ from :: Elf32 (ref $ var) => $ to :: Elf32 ($ body) , # [cfg (feature = "elf")] $ from :: Elf64 (ref $ var) => $ to :: Elf64 ($ body) , # [cfg (feature = "macho")] $ from :: MachO32 (ref $ var) => $ to :: MachO32 ($ body) , # [cfg (feature = "macho")] $ from :: MachO64 (ref $ var) => $ to :: MachO64 ($ body) , # [cfg (feature = "pe")] $ from :: Pe32 (ref $ var) => $ to :: Pe32 ($ body) , # [cfg (feature = "pe")] $ from :: Pe64 (ref $ var) => $ to :: Pe64 ($ body) , # [cfg (feature = "wasm")] $ from :: Wasm (ref $ var) => $ to :: Wasm ($ body) , # [cfg (feature = "xcoff")] $ from :: Xcoff32 (ref $ var) => $ to :: Xcoff32 ($ body) , # [cfg (feature = "xcoff")] $ from :: Xcoff64 (ref $ var) => $ to :: Xcoff64 ($ body) , } } ; }
    };
}

map_inner!()