macro_rules! deps {
    () => {
        CrateBuilder!();
    };
}

macro_rules! CrateGraphBuilder {
    () => {
        deps!();
        # [derive (Default , Clone)] pub struct CrateGraphBuilder { arena : Arena < CrateBuilder > , }
    };
}

CrateGraphBuilder!()