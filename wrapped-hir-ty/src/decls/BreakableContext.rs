macro_rules! deps {
    () => {
        BreakableKind!();
    };
}

macro_rules! BreakableContext {
    () => {
        deps!();
        # [derive (Clone , Debug)] struct BreakableContext < 'db > { # [doc = " Whether this context contains at least one break expression."] may_break : bool , # [doc = " The coercion target of the context."] coerce : Option < DynamicCoerceMany < 'db > > , # [doc = " The optional label of the context."] label : Option < LabelId > , kind : BreakableKind , }
    };
}

BreakableContext!()