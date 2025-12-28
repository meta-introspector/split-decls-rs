macro_rules! deps {
    () => {
        NavigationTarget!();
        RunnableKind!();
        UpdateTest!();
    };
}

macro_rules! Runnable {
    () => {
        deps!();
        # [derive (Debug , Clone , Hash , PartialEq , Eq , UpmapFromRaFixture)] pub struct Runnable { pub use_name_in_title : bool , pub nav : NavigationTarget , pub kind : RunnableKind , pub cfg : Option < CfgExpr > , pub update_test : UpdateTest , }
    };
}

Runnable!();