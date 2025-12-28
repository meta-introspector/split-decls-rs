macro_rules! deps {
    () => {
        FieldValue!();
        Field!();
        Context!();
        ObjectAccessor!();
    };
}

macro_rules! ResolverContext {
    () => {
        deps!();
        # [doc = " A context for resolver function"] pub struct ResolverContext < 'a > { # [doc = " GraphQL context"] pub ctx : & 'a Context < 'a > , # [doc = " Field arguments"] pub args : ObjectAccessor < 'a > , # [doc = " Parent value"] pub parent_value : & 'a FieldValue < 'a > , }
    };
}

ResolverContext!()