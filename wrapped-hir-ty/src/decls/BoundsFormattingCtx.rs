macro_rules! deps {
    () => {
        AliasTy!();
    };
}

macro_rules! BoundsFormattingCtx {
    () => {
        deps!();
        # [derive (Default)] enum BoundsFormattingCtx < 'db > { Entered { # [doc = " We can have recursive bounds like the following case:"] # [doc = " ```ignore"] # [doc = " where"] # [doc = "     T: Foo,"] # [doc = "     T::FooAssoc: Baz<<T::FooAssoc as Bar>::BarAssoc> + Bar"] # [doc = " ```"] # [doc = " So, record the projection types met while formatting bounds and"] projection_tys_met : FxHashSet < AliasTy < 'db > > , } , # [default] Exited , }
    };
}

BoundsFormattingCtx!()