macro_rules! deps {
    () => {
        Const!();
    };
}

macro_rules! EvaluatedConst {
    () => {
        deps!();
        pub struct EvaluatedConst < 'db > { def : DefWithBodyId , const_ : hir_ty :: next_solver :: Const < 'db > , ty : Ty < 'db > , }
    };
}

EvaluatedConst!();