macro_rules! ZeroVecAttrs {
    () => {
        # [derive (Default , Clone)] pub struct ZeroVecAttrs { pub skip_kv : bool , pub skip_ord : bool , pub skip_toowned : bool , pub skip_from : bool , pub serialize : bool , pub deserialize : bool , pub debug : bool , pub hash : bool , pub vzv_format : Option < TokenStream2 > , }
    };
}

ZeroVecAttrs!()