macro_rules! deps {
    () => {
        DeclArgs!();
    };
}

macro_rules! impl_2 {
    () => {
        deps!();
        impl Parse for DeclArgs { fn parse (input : ParseStream) -> SynResult < Self > { let mut args = DeclArgs { node_type : None , name : None , vis : None , hash : None , extra : vec ! [] , } ; if input . peek (syn :: Ident) { let ident : Ident = input . parse () ? ; args . node_type = Some (ident . to_string ()) ; if input . peek (syn :: Token ! [,]) { input . parse :: < syn :: Token ! [,] > () ? ; } } while ! input . is_empty () { let key : Ident = input . parse () ? ; input . parse :: < syn :: Token ! [=] > () ? ; let value : LitStr = input . parse () ? ; match key . to_string () . as_str () { "name" => args . name = Some (value . value ()) , "vis" => args . vis = Some (value . value ()) , "hash" => args . hash = Some (value . value ()) , other => args . extra . push ((other . to_string () , value . value ())) , } if input . peek (syn :: Token ! [,]) { input . parse :: < syn :: Token ! [,] > () ? ; } } Ok (args) } }
    };
}

impl_2!()