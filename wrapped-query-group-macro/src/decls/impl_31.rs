macro_rules! deps {
    () => {
        Cycle!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        impl Parse for Cycle { fn parse (input : ParseStream < '_ >) -> syn :: Result < Self > { let options = Punctuated :: < Option , Token ! [,] > :: parse_terminated (input) ? ; let mut cycle_fn = None ; let mut cycle_initial = None ; let mut cycle_result = None ; for option in options { let name = option . name . to_string () ; match & * name { "cycle_fn" => { if cycle_fn . is_some () { return Err (syn :: Error :: new_spanned (& option . name , "duplicate option")) ; } cycle_fn = Some ((option . name , option . value)) ; } "cycle_initial" => { if cycle_initial . is_some () { return Err (syn :: Error :: new_spanned (& option . name , "duplicate option")) ; } cycle_initial = Some ((option . name , option . value)) ; } "cycle_result" => { if cycle_result . is_some () { return Err (syn :: Error :: new_spanned (& option . name , "duplicate option")) ; } cycle_result = Some ((option . name , option . value)) ; } _ => { return Err (syn :: Error :: new_spanned (& option . name , "unknown cycle option. Accepted values: `cycle_result`, `cycle_fn`, `cycle_initial`" ,)) ; } } } return Ok (Self { cycle_fn , cycle_initial , cycle_result }) ; struct Option { name : syn :: Ident , value : Path , } impl Parse for Option { fn parse (input : ParseStream < '_ >) -> syn :: Result < Self > { let name = input . parse () ? ; input . parse :: < Token ! [=] > () ? ; let value = input . parse () ? ; Ok (Self { name , value }) } } } }
    };
}

impl_31!()