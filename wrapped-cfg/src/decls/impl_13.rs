macro_rules! deps {
    () => {
        InactiveReason!();
        CfgDiff!();
        DnfExpr!();
        CfgExpr!();
        CfgOptions!();
        Builder!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl DnfExpr { pub fn new (expr : & CfgExpr) -> Self { let builder = Builder { expr : DnfExpr { conjunctions : Vec :: new () } } ; builder . lower (expr) } # [doc = " Computes a list of present or absent atoms in `opts` that cause this expression to evaluate"] # [doc = " to `false`."] # [doc = ""] # [doc = " Note that flipping a subset of these atoms might be sufficient to make the whole expression"] # [doc = " evaluate to `true`. For that, see `compute_enable_hints`."] # [doc = ""] # [doc = " Returns `None` when `self` is already true, or contains errors."] pub fn why_inactive (& self , opts : & CfgOptions) -> Option < InactiveReason > { let mut res = InactiveReason { enabled : Vec :: new () , disabled : Vec :: new () } ; for conj in & self . conjunctions { let mut conj_is_true = true ; for lit in & conj . literals { let atom = lit . var . as_ref () ? ; let enabled = opts . enabled . contains (atom) ; if lit . negate == enabled { conj_is_true = false ; if enabled { res . enabled . push (atom . clone ()) ; } else { res . disabled . push (atom . clone ()) ; } } } if conj_is_true { return None ; } } res . enabled . sort_unstable () ; res . enabled . dedup () ; res . disabled . sort_unstable () ; res . disabled . dedup () ; Some (res) } # [doc = " Returns `CfgDiff` objects that would enable this directive if applied to `opts`."] pub fn compute_enable_hints < 'a > (& 'a self , opts : & 'a CfgOptions ,) -> impl Iterator < Item = CfgDiff > + 'a { self . conjunctions . iter () . filter_map (move | conj | { let mut enable = FxHashSet :: default () ; let mut disable = FxHashSet :: default () ; for lit in & conj . literals { let atom = lit . var . as_ref () ? ; let enabled = opts . enabled . contains (atom) ; if lit . negate && enabled { disable . insert (atom . clone ()) ; } if ! lit . negate && ! enabled { enable . insert (atom . clone ()) ; } } for lit in & conj . literals { let atom = lit . var . as_ref () ? ; let enabled = enable . contains (atom) || (opts . enabled . contains (atom) && ! disable . contains (atom)) ; if enabled == lit . negate { return None ; } } if enable . is_empty () && disable . is_empty () { return None ; } let mut diff = CfgDiff { enable : enable . into_iter () . collect () , disable : disable . into_iter () . collect () , } ; diff . enable . sort_unstable () ; diff . disable . sort_unstable () ; Some (diff) }) } }
    };
}

impl_13!();