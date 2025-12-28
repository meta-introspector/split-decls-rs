macro_rules! get_dllimport {
    () => {
        pub (crate) fn get_dllimport < 'tcx > (tcx : TyCtxt < 'tcx > , id : DefId , name : & str ,) -> Option < & 'tcx DllImport > { tcx . native_library (id) . and_then (| lib | lib . dll_imports . iter () . find (| di | di . name . as_str () == name)) }
    };
}

get_dllimport!();