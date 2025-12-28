macro_rules! deps {
    () => {
        Graph!();
    };
}

macro_rules! Generator {
    () => {
        deps!();
        # [doc = " A graph generator of “all” graphs of a particular size."] # [doc = ""] # [doc = " ***Unstable: API may change at any time.*** Depends on `feature = \"generate\"`."] pub struct Generator < Ty > { acyclic : bool , selfloops : bool , nodes : usize , # [doc = " number of possible edges"] nedges : usize , # [doc = " current edge bitmap"] bits : u64 , g : Graph < () , () , Ty > , }
    };
}

Generator!();