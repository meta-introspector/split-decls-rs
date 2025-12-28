macro_rules! gen_keyid {
    () => {
        # [doc = " Generate a suitable mockall::Key generic paramter from any Generics"] fn gen_keyid (g : & Generics) -> impl ToTokens { match g . params . len () { 0 => quote ! (< () >) , 1 => { let (_ , tg , _) = g . split_for_impl () ; quote ! (# tg) } , _ => { let tps = g . type_params () . map (| tp | tp . ident . clone ()) . collect :: < Punctuated :: < Ident , Token ! [,] > > () ; quote ! (< (# tps) >) } } }
    };
}

gen_keyid!();