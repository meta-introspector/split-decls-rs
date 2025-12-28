macro_rules! deps {
    () => {
        DefMap!();
        DefDatabase!();
    };
}

macro_rules! crate_def_map {
    () => {
        deps!();
        # [inline] pub fn crate_def_map (db : & dyn DefDatabase , crate_id : Crate) -> & DefMap { crate_local_def_map (db , crate_id) . def_map (db) }
    };
}

crate_def_map!();