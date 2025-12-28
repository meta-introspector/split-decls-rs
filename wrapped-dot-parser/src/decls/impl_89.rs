macro_rules! deps {
    () => {
        Graph!();
    };
}

macro_rules! impl_89 {
    () => {
        deps!();
        # [cfg (feature = "display")] impl < A > Display for Graph < A > where A : Display , { fn fmt (& self , f : & mut Formatter < '_ >) -> Result < () , std :: fmt :: Error > { if self . strict { write ! (f , "strict ") ? ; } if self . is_digraph { write ! (f , "digraph ") ? ; } else { write ! (f , "graph ") ? ; } if let Some (name) = & self . name { write ! (f , "{}" , name) ? ; } writeln ! (f , "{{") ? ; for stmt in & self . attr { writeln ! (f , "{}" , stmt) ? ; } for ideq in & self . ideqs { writeln ! (f , "{}" , ideq) ? ; } write ! (f , "{}" , self . nodes) ? ; write ! (f , "{}" , self . edges) ? ; writeln ! (f , "}}") ? ; Result :: Ok (()) } }
    };
}

impl_89!()