macro_rules! deps {
    () => {
        HirDatabase!();
    };
}

macro_rules! try_const_usize {
    () => {
        deps!();
        pub fn try_const_usize < 'db > (db : & 'db dyn HirDatabase , c : Const < 'db >) -> Option < u128 > { match c . kind () { ConstKind :: Param (_) => None , ConstKind :: Infer (_) => None , ConstKind :: Bound (_ , _) => None , ConstKind :: Placeholder (_) => None , ConstKind :: Unevaluated (unevaluated_const) => match unevaluated_const . def { SolverDefId :: ConstId (id) => { let subst = unevaluated_const . args ; let ec = db . const_eval (id , subst , None) . ok () ? ; try_const_usize (db , ec) } SolverDefId :: StaticId (id) => { let ec = db . const_eval_static (id) . ok () ? ; try_const_usize (db , ec) } _ => unreachable ! () , } , ConstKind :: Value (val) => Some (u128 :: from_le_bytes (pad16 (& val . value . inner () . memory , false))) , ConstKind :: Error (_) => None , ConstKind :: Expr (_) => None , } }
    };
}

try_const_usize!();