macro_rules! deps {
    () => {
        AttributeLintKind!();
    };
}

macro_rules! AttributeLint {
    () => {
        deps!();
        # [derive (Clone , Debug , HashStable_Generic)] pub struct AttributeLint < Id > { pub id : Id , pub span : Span , pub kind : AttributeLintKind , }
    };
}

AttributeLint!()