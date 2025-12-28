macro_rules! deps {
    () => {
        Conversion!();
    };
}

macro_rules! convert {
    () => {
        deps!();
        fn convert (inputs : & Punctuated < FnArg , Token ! [,] > , ty_conversions : & HashMap < Ident , Conversion > ,) -> (bool , Punctuated < FnArg , Token ! [,] > , Punctuated < Expr , Token ! [,] > , bool) { let mut has_conversion_in_effect = false ; let mut argtypes = Punctuated :: new () ; let mut argexprs = Punctuated :: new () ; let mut has_self = false ; inputs . iter () . enumerate () . for_each (| (i , input) | match input . clone () { FnArg :: Receiver (receiver) => { has_self = true ; argtypes . push (FnArg :: Receiver (receiver)) ; } FnArg :: Typed (mut pat_type) => { let pat_ident = match & mut * pat_type . pat { Pat :: Ident (pat_ident) if pat_ident . by_ref . is_none () && pat_ident . subpat . is_none () => pat_ident , _ => { * pat_type . pat = Pat :: Ident (PatIdent { ident : Ident :: new (& format ! ("arg_{i}_gen_by_momo_") , proc_macro2 :: Span :: call_site ()) , attrs : Default :: default () , by_ref : None , mutability : None , subpat : None , }) ; if let Pat :: Ident (pat_ident) = & mut * pat_type . pat { pat_ident } else { panic ! () } } } ; pat_ident . mutability = None ; let ident = & pat_ident . ident ; let to_expr = | | parse_quote ! (# ident) ; match * pat_type . ty { Type :: ImplTrait (TypeImplTrait { ref bounds , .. }) => { if let Some (conv) = parse_bounds (bounds) { has_conversion_in_effect = true ; argexprs . push (conv . conversion_expr (ident)) ; if let Conversion :: AsMut = conv { pat_ident . mutability = Some (Default :: default ()) ; } } else { argexprs . push (to_expr ()) ; } } Type :: Path (..) => { if let Some (conv) = parse_bounded_type (& pat_type . ty) . and_then (| ident | ty_conversions . get (& ident)) { has_conversion_in_effect = true ; argexprs . push (conv . conversion_expr (ident)) ; if let Conversion :: AsMut = conv { pat_ident . mutability = Some (Default :: default ()) ; } } else { argexprs . push (to_expr ()) ; } } _ => { argexprs . push (to_expr ()) ; } } argtypes . push (FnArg :: Typed (pat_type)) ; } }) ; (has_conversion_in_effect , argtypes , argexprs , has_self) }
    };
}

convert!()