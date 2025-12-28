macro_rules! deps {
    () => {
        RPathConfig!();
    };
}

macro_rules! get_rpath_relative_to_output {
    () => {
        deps!();
        fn get_rpath_relative_to_output (config : & RPathConfig < '_ > , lib : & Path) -> OsString { let prefix = if config . is_like_darwin { "@loader_path" } else { "$ORIGIN" } ; let lib = lib . parent () . unwrap () ; let output = config . out_filename . parent () . unwrap () ; let lib = if lib == Path :: new ("") { Path :: new (".") } else { lib } ; let output = if output == Path :: new ("") { Path :: new (".") } else { output } ; let lib = try_canonicalize (lib) . unwrap () ; let output = try_canonicalize (output) . unwrap () ; let relative = path_relative_from (& lib , & output) . unwrap_or_else (| | panic ! ("couldn't create relative path from {output:?} to {lib:?}")) ; let mut rpath = OsString :: from (prefix) ; rpath . push ("/") ; rpath . push (relative) ; rpath }
    };
}

get_rpath_relative_to_output!();