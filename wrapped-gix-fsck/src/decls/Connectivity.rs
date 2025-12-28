macro_rules! Connectivity {
    () => {
        # [doc = " Perform a connectivity check."] pub struct Connectivity < T , F > where T : FindExt + Exists , F : FnMut (& ObjectId , Kind) , { # [doc = " ODB handle to use for the check"] db : T , # [doc = " Closure to invoke when a missing object is encountered"] missing_cb : F , # [doc = " Set of Object IDs already (or about to be) scanned during the check"] seen : HashSet , # [doc = " A buffer to keep a single object at a time."] buf : Vec < u8 > , }
    };
}

Connectivity!();