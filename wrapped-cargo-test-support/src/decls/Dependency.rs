macro_rules! Dependency {
    () => {
        # [doc = " Published package dependency builder, see [`Package::add_dep`]"] # [derive (Clone)] pub struct Dependency { name : String , vers : String , kind : String , artifact : Option < String > , bindep_target : Option < String > , lib : bool , target : Option < String > , features : Vec < String > , registry : Option < String > , package : Option < String > , optional : bool , default_features : bool , public : bool , }
    };
}

Dependency!();