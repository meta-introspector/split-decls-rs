macro_rules! deps {
    () => {
        CodegenCx!();
    };
}

macro_rules! mangled_name_of_instance {
    () => {
        deps!();
        pub (crate) fn mangled_name_of_instance < 'a , 'tcx > (cx : & CodegenCx < 'a , 'tcx > , instance : Instance < 'tcx > ,) -> ty :: SymbolName < 'tcx > { cx . tcx . symbol_name (instance) }
    };
}

mangled_name_of_instance!()