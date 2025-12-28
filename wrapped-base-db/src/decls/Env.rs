macro_rules! Env {
    () => {
        # [derive (Default , Clone , PartialEq , Eq)] pub struct Env { entries : FxHashMap < String , String > , }
    };
}

Env!();