macro_rules! deps {
    () => {
        FindPathConfig!();
        DefMap!();
        DefDatabase!();
        PrefixKind!();
        ModuleId!();
    };
}

macro_rules! FindPathCtx {
    () => {
        deps!();
        struct FindPathCtx < 'db > { db : & 'db dyn DefDatabase , prefix : PrefixKind , cfg : FindPathConfig , ignore_local_imports : bool , is_std_item : bool , from : ModuleId , from_def_map : & 'db DefMap , fuel : Cell < usize > , }
    };
}

FindPathCtx!()