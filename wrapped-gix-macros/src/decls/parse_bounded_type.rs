macro_rules! parse_bounded_type {
    () => {
        fn parse_bounded_type (ty : & Type) -> Option < Ident > { match & ty { Type :: Path (TypePath { qself : None , path }) if path . segments . len () == 1 => Some (path . segments [0] . ident . clone ()) , _ => None , } }
    };
}

parse_bounded_type!();