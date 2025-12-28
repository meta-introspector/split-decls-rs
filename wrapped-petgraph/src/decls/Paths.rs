macro_rules! Paths {
    () => {
        # [derive (Debug , Clone)] pub struct Paths < NodeId , EdgeWeight > { pub distances : Vec < EdgeWeight > , pub predecessors : Vec < Option < NodeId > > , }
    };
}

Paths!();