macro_rules! deps {
    () => {
        ImportMap!();
        ItemInNs!();
    };
}

macro_rules! impl_425 {
    () => {
        deps!();
        impl fmt :: Debug for ImportMap { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut importable_names : Vec < _ > = self . item_to_info_map . iter () . map (| (item , (infos , _)) | { let l = infos . len () ; match item { ItemInNs :: Types (it) => format ! ("- {it:?} (t) [{l}]" ,) , ItemInNs :: Values (it) => format ! ("- {it:?} (v) [{l}]" ,) , ItemInNs :: Macros (it) => format ! ("- {it:?} (m) [{l}]" ,) , } }) . collect () ; importable_names . sort () ; f . write_str (& importable_names . join ("\n")) } }
    };
}

impl_425!();