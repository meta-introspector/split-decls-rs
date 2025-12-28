macro_rules! lifetimes_to_generics {
    () => {
        # [doc = " Transform a Vec of lifetimes into a Generics"] fn lifetimes_to_generics (lv : & Punctuated < LifetimeParam , Token ! [,] >) -> Generics { if lv . is_empty () { Generics :: default () } else { let params = lifetimes_to_generic_params (lv) ; Generics { lt_token : Some (Token ! [<] (lv [0] . span ())) , gt_token : Some (Token ! [>] (lv [0] . span ())) , params , where_clause : None } } }
    };
}

lifetimes_to_generics!();