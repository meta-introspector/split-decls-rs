macro_rules! DynMap {
    () => {
        # [derive (Default)] pub struct DynMap { pub (crate) map : Map , }
    };
}

DynMap!();