macro_rules! for_each_exported_symbols_include_dep {
    () => {
        fn for_each_exported_symbols_include_dep < 'tcx > (tcx : TyCtxt < 'tcx > , crate_type : CrateType , mut callback : impl FnMut (ExportedSymbol < 'tcx > , SymbolExportInfo , CrateNum) ,) { let formats = tcx . dependency_formats (()) ; let deps = & formats [& crate_type] ; for (cnum , dep_format) in deps . iter_enumerated () { if * dep_format == Linkage :: Static { for & (symbol , info) in tcx . exported_non_generic_symbols (cnum) . iter () { callback (symbol , info , cnum) ; } for & (symbol , info) in tcx . exported_generic_symbols (cnum) . iter () { callback (symbol , info , cnum) ; } } } }
    };
}

for_each_exported_symbols_include_dep!()