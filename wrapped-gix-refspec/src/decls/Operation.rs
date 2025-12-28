macro_rules! deps {
    () => {
        Push!();
        Fetch!();
    };
}

macro_rules! Operation {
    () => {
        deps!();
        # [doc = " Define how the parsed refspec should be used."] # [derive (PartialOrd , Ord , PartialEq , Eq , Copy , Clone , Hash , Debug)] pub enum Operation { # [doc = " The `src` side is local and the `dst` side is remote."] Push , # [doc = " The `src` side is remote and the `dst` side is local."] Fetch , }
    };
}

Operation!()