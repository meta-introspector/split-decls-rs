macro_rules! deps {
    () => {
        Dfs!();
        DfsSpace!();
        VisitMap!();
    };
}

macro_rules! impl_484 {
    () => {
        deps!();
        impl < N , VM > Default for DfsSpace < N , VM > where VM : VisitMap < N > + Default , { fn default () -> Self { DfsSpace { dfs : Dfs { stack : < _ > :: default () , discovered : < _ > :: default () , } , } } }
    };
}

impl_484!()