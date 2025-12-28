macro_rules! deps {
    () => {
        Graph!();
    };
}

macro_rules! impl_90 {
    () => {
        deps!();
        impl < A > Graph < A > { # [doc = " Filter and map attributes. The main intended usage of this function is"] # [doc = " to convert attributes as `&'a str` into an enum, e.g."] # [doc = " to convert `[\"label\"=\"whatever\", \"color\"=\"foo\"]` into"] # [doc = " `[Attr::Label(whatever), Attr::Color(foo)]`."] # [doc = ""] # [doc = " To take into account non-standard attributes, the `Attr` enum has to be"] # [doc = " provided by the user."] pub fn filter_map < F , B > (self , f : F) -> Graph < B > where F : Fn (A) -> Option < B > , { let new_attr = self . attr . into_iter () . filter_map (| attr_stmt | attr_stmt . filter_map (& f)) . collect () ; let new_nodes = self . nodes . map (& f) ; let new_edges = self . edges . map (& f) ; Graph { strict : self . strict , is_digraph : self . is_digraph , name : self . name , attr : new_attr , nodes : new_nodes , edges : new_edges , ideqs : self . ideqs , } } }
    };
}

impl_90!()