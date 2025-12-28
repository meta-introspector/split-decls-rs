macro_rules! defraw {
    () => {
        macro_rules ! defraw { ($ ty : ident , $ find : ident , $ start : ident , $ end : ident , $ ($ needles : ident) ,+) => { { use crate :: arch :: wasm32 :: simd128 :: memchr ::$ ty ; debug ! ("chose simd128 for {}" , stringify ! ($ ty)) ; debug_assert ! ($ ty :: is_available ()) ; $ ty :: new_unchecked ($ ($ needles) ,+) .$ find ($ start , $ end) } } }
    };
}

defraw!();