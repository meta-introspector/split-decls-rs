macro_rules! deps {
    () => {
        Type!();
        Module!();
        Const!();
        EvaluatedConst!();
    };
}

macro_rules! impl_297 {
    () => {
        deps!();
        impl Const { pub fn module (self , db : & dyn HirDatabase) -> Module { Module { id : self . id . module (db) } } pub fn name (self , db : & dyn HirDatabase) -> Option < Name > { db . const_signature (self . id) . name . clone () } pub fn value (self , db : & dyn HirDatabase) -> Option < ast :: Expr > { self . source (db) ? . value . body () } pub fn ty (self , db : & dyn HirDatabase) -> Type < '_ > { Type :: from_value_def (db , self . id) } # [doc = " Evaluate the constant."] pub fn eval (self , db : & dyn HirDatabase) -> Result < EvaluatedConst < '_ > , ConstEvalError < '_ > > { let interner = DbInterner :: new_with (db , None , None) ; let ty = db . value_ty (self . id . into ()) . unwrap () . instantiate_identity () ; db . const_eval (self . id , GenericArgs :: new_from_iter (interner , []) , None) . map (| it | EvaluatedConst { const_ : it , def : self . id . into () , ty }) } }
    };
}

impl_297!();