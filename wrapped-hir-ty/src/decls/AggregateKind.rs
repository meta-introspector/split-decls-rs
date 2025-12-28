macro_rules! AggregateKind {
    () => {
        # [derive (Debug , PartialEq , Eq , Clone)] pub enum AggregateKind < 'db > { # [doc = " The type is of the element"] Array (Ty < 'db >) , # [doc = " The type is of the tuple"] Tuple (Ty < 'db >) , Adt (VariantId , GenericArgs < 'db >) , Union (UnionId , FieldId) , Closure (Ty < 'db >) , }
    };
}

AggregateKind!();