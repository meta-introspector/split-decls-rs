macro_rules! deps {
    () => {
        Conjunction!();
    };
}

macro_rules! DnfExpr {
    () => {
        deps!();
        # [doc = " A `#[cfg]` directive in Disjunctive Normal Form (DNF)."] pub struct DnfExpr { conjunctions : Vec < Conjunction > , }
    };
}

DnfExpr!()