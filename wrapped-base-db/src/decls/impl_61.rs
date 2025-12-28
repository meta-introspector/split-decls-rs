macro_rules! deps {
    () => {
        CrateBuilderId!();
        CrateDisplayName!();
        CyclicDependenciesError!();
    };
}

macro_rules! impl_61 {
    () => {
        deps!();
        impl fmt :: Display for CyclicDependenciesError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let render = | (id , name) : & (CrateBuilderId , Option < CrateDisplayName >) | match name { Some (it) => format ! ("{it}({id:?})") , None => format ! ("{id:?}") , } ; let path = self . path . iter () . rev () . map (render) . collect :: < Vec < String > > () . join (" -> ") ; write ! (f , "cyclic deps: {} -> {}, alternative path: {}" , render (self . from ()) , render (self . to ()) , path) } }
    };
}

impl_61!();