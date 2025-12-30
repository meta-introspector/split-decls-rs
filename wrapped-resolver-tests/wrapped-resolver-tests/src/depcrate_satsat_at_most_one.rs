// Generated macro for sat_at_most_one (function)
macro_rules! Depcrate_satsat_at_most_one {
() => {
// Module: crate::sat
// Provides: {"sat_at_most_one"}
// Dependencies: {}
# [doc = " At this point is possible to select every version of every package."] # [doc = ""] # [doc = " So we need to mark certain versions as incompatible with each other."] # [doc = " We could add a clause not A, not B for all A and B that are incompatible,"] fn sat_at_most_one (solver : & mut varisat :: Solver < '_ > , vars : & [varisat :: Var]) { if vars . len () <= 1 { return ; } else if vars . len () == 2 { solver . add_clause (& [vars [0] . negative () , vars [1] . negative ()]) ; return ; } else if vars . len () == 3 { solver . add_clause (& [vars [0] . negative () , vars [1] . negative ()]) ; solver . add_clause (& [vars [0] . negative () , vars [2] . negative ()]) ; solver . add_clause (& [vars [1] . negative () , vars [2] . negative ()]) ; return ; } let bits : Vec < varisat :: Var > = solver . new_var_iter (log_bits (vars . len ())) . collect () ; for (i , p) in vars . iter () . enumerate () { for b in 0 .. bits . len () { solver . add_clause (& [p . negative () , bits [b] . lit (((1 << b) & i) > 0)]) ; } } }
};
}
