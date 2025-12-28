macro_rules! deps {
    () => {
        HirDatabase!();
    };
}

macro_rules! try_const_isize {
    () => {
        deps!();
        pub fn try_const_isize < 'db > (db : & 'db dyn HirDatabase , c : & Const < 'db >) -> Option < i128 > { match (* c) . kind () { ConstKind :: Param (_) => None , ConstKind :: Infer (_) => None , ConstKind :: Bound (_ , _) => None , ConstKind :: Placeholder (_) => None , ConstKind :: Unevaluated (unevaluated_const) => match unevaluated_const . def { SolverDefId :: ConstId (id) => { let subst = unevaluated_const . args ; let ec = db . const_eval (id , subst , None) . ok () ? ; try_const_isize (db , & ec) } SolverDefId :: StaticId (id) => { let ec = db . const_eval_static (id) . ok () ? ; try_const_isize (db , & ec) } _ => unreachable ! () , } , ConstKind :: Value (val) => Some (i128 :: from_le_bytes (pad16 (& val . value . inner () . memory , true))) , ConstKind :: Error (_) => None , ConstKind :: Expr (_) => None , } }
    };
}

try_const_isize!()