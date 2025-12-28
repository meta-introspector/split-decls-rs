macro_rules! deps {
    () => {
        Query!();
        LibraryRoots!();
        SymbolIndex!();
        LocalRoots!();
        RootDatabase!();
    };
}

macro_rules! world_symbols {
    () => {
        deps!();
        pub fn world_symbols (db : & RootDatabase , query : Query) -> Vec < FileSymbol > { let _p = tracing :: info_span ! ("world_symbols" , query = ? query . query) . entered () ; let indices : Vec < _ > = if query . libs { LibraryRoots :: get (db) . roots (db) . par_iter () . for_each_with (db . clone () , | snap , & root | _ = SymbolIndex :: library_symbols (snap , root)) ; LibraryRoots :: get (db) . roots (db) . iter () . map (| & root | SymbolIndex :: library_symbols (db , root)) . collect () } else { let mut crates = Vec :: new () ; for & root in LocalRoots :: get (db) . roots (db) . iter () { crates . extend (db . source_root_crates (root) . iter () . copied ()) } crates . par_iter () . for_each_with (db . clone () , | snap , & krate | _ = crate_symbols (snap , krate . into ())) ; let indices : Vec < _ > = crates . into_iter () . map (| krate | crate_symbols (db , krate . into ())) . collect () ; indices . iter () . flat_map (| indices | indices . iter () . cloned ()) . collect () } ; let mut res = vec ! [] ; query . search :: < () > (& indices , | f | { res . push (f . clone ()) ; ControlFlow :: Continue (()) }) ; res }
    };
}

world_symbols!();